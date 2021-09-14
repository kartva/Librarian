# Setup:

```bash
git clone https://github.com/DesmondWillowbrook/Server_Web_Library_Base_Compositions.git

# Pulling in frontend repo.
git submodule init 
git submodule update

docker pull desmondwillowbrook/server-web-library-base-compositions
docker run -dp 8186:8186 desmondwillowbrook/server-web-library-base-compositions
```

Refer to Dockerfile for actual setup instructions.
