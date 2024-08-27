use anyhow::Result;
use url::Url;

pub fn is_gitpod() -> bool {
    return std::env::var("GITPOD_WORKSPACE_ID").is_ok() &&
      std::env::var("GITPOD_WORKSPACE_CLUSTER_HOST").is_ok()
}

pub fn translate_gitpod_url(urlstr: &str) -> Result<String> {
    let mut url = Url::parse(urlstr)?;
    
    if url.port().is_none() {
        return Err(anyhow::anyhow!("port is not specified"));
    }
    if !is_gitpod() {
        return Err(anyhow::anyhow!("Not in Gitpod environment"));
    }

    if url.scheme().starts_with("ws") {
        url.set_scheme("wss").map_err(|_| anyhow::anyhow!("Failed to set scheme for URL: {}", urlstr))?;
    }

    let workspace_id = std::env::var("GITPOD_WORKSPACE_ID")?;
    let cluster_host = std::env::var("GITPOD_WORKSPACE_CLUSTER_HOST")?;
    let hostname = format!("{}-{}.{}", url.port().unwrap(), workspace_id, cluster_host);
    
    url.set_host(Some(&hostname)).map_err(|_| anyhow::anyhow!("Failed to set hostname for URL: {}", urlstr))?;
    url.set_port(None).map_err(|_| anyhow::anyhow!("Failed to remove port for URL: {}", urlstr))?;

    Ok(url.to_string())
}
