# Deploying Rustegrate to Azure Container Instances

This guide provides step-by-step instructions for deploying the Rustegrate application to Azure Container Instances using GitHub Actions for continuous deployment.

## Prerequisites

- Azure account with an active subscription
- GitHub repository with your Rust application code

## Step 1: Create Azure Resources

### Option 1: Using Azure Portal

1. Log in to the [Azure Portal](https://portal.azure.com)
2. Create a new Resource Group:
   - Click on "Resource groups" in the left navigation
   - Click "Create"
   - Name it "rustegrate-rg"
   - Select a region close to your location
   - Click "Review + create" then "Create"

3. Create a Container Instance (Optional - can be done via GitHub Actions):
   - Navigate to your resource group
   - Click "Create" and search for "Container Instance"
   - Fill in the basic details:
     - Container name: rustegrate-container
     - Region: East US (or your preferred region)
     - Image source: Docker Hub or other registry
     - Image: ghcr.io/yourusername/rustegrate:latest
   - Set networking options:
     - DNS name label: rustegrate-api
     - Ports: 8080 TCP
   - Click "Review + create" then "Create"

### Option 2: Using Azure CLI (Recommended)

```bash
# Login to Azure
az login

# Create a resource group
az group create --name rustegrate-rg --location eastus

# Create a service principal for GitHub Actions
az ad sp create-for-rbac --name "rustegrate-github" --role contributor \
  --scopes /subscriptions/{your-subscription-id}/resourceGroups/rustegrate-rg --sdk-auth

# Note: Save the JSON output - you'll need it for GitHub Secrets
```

## Step 2: Configure GitHub Repository

1. Add the following secrets to your GitHub repository:
   - Go to Settings > Secrets and variables > Actions
   - Add the following secrets:
     - `AZURE_CREDENTIALS`: The entire JSON output from the service principal creation
     - `GITHUB_TOKEN`: This is automatically provided by GitHub

## Step 3: Update GitHub Actions Workflow

Create a file at `.github/workflows/azure-deploy.yml` with the following content:

```yaml
name: Deploy to Azure Container Instances

on:
  push:
    branches: [ main, master ]
  workflow_dispatch:

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v2
      
    - name: Set up Docker Buildx
      uses: docker/setup-buildx-action@v1
      
    - name: Log in to GitHub Container Registry
      uses: docker/login-action@v1
      with:
        registry: ghcr.io
        username: ${{ github.repository_owner }}
        password: ${{ secrets.GITHUB_TOKEN }}
        
    - name: Build and push Docker image
      uses: docker/build-push-action@v2
      with:
        context: .
        push: true
        tags: |
          ghcr.io/${{ github.repository }}:${{ github.sha }}
          ghcr.io/${{ github.repository }}:latest
          
    - name: Login to Azure
      uses: azure/login@v1
      with:
        creds: ${{ secrets.AZURE_CREDENTIALS }}
        
    - name: Deploy to Azure Container Instances
      run: |
        az container create \
          --resource-group rustegrate-rg \
          --name rustegrate-container \
          --image ghcr.io/${{ github.repository }}:${{ github.sha }} \
          --dns-name-label rustegrate-api \
          --ports 8080 \
          --protocol TCP \
          --environment-variables HOST=0.0.0.0 PORT=8080 LOG_LEVEL=info \
          --registry-login-server ghcr.io \
          --registry-username ${{ github.repository_owner }} \
          --registry-password ${{ secrets.GITHUB_TOKEN }} \
          --restart-policy OnFailure \
          --replace
          
    - name: Azure logout
      run: |
        az logout
```

## Step 4: Monitor Deployment

1. Push your changes to the main/master branch to trigger the deployment
2. Monitor the workflow progress on GitHub under the Actions tab
3. Once completed, your app should be available at:
   ```
   http://rustegrate-api.{region}.azurecontainer.io:8080
   ```
   For example: http://rustegrate-api.eastus.azurecontainer.io:8080

## Cost Management

Azure Container Instances charges only for the time your container runs:
- Approximately $0.0025 per vCPU per hour
- Approximately $0.0003 per GB memory per hour

For a small app like Rustegrate:
- 1 vCPU, 1.5GB memory configuration
- Running continuously: ~$3-5 per month
- Running occasionally: Much less

This is more cost-effective than App Service Basic tier and provides flexibility for your application.

## Troubleshooting

- **Container not starting**: Check the container logs in Azure Portal > Container Instances > rustegrate-container > Containers > Logs
- **Deployment failing**: Verify that GitHub Secrets are correctly configured
- **Network issues**: Ensure port 8080 is properly exposed in both the Dockerfile and Container Instance configuration

## Cleaning Up Resources

To avoid unexpected charges, delete the resource group when you're done:

```bash
az group delete --name rustegrate-rg --yes
```

Or use the Azure Portal:
1. Navigate to the "rustegrate-rg" resource group
2. Click "Delete resource group"
3. Confirm the deletion by typing the resource group name

## Additional Resources

- [Azure Container Instances Documentation](https://docs.microsoft.com/en-us/azure/container-instances/)
- [GitHub Actions for Azure](https://github.com/Azure/actions)
- [GitHub Container Registry Documentation](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry) 