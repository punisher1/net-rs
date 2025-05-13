use anyhow::{Context, Result};
use rustls::{
    ClientConfig, RootCertStore, ServerConfig,
    crypto::ring::sign::RsaSigningKey,
};
use std::{
    fs::File,
    io::BufReader,
    path::Path,
    sync::Arc,
};
use rustls_pemfile::{certs, pkcs8_private_keys};

/// TLS 配置管理
pub struct TlsConfig;

impl TlsConfig {
    /// 创建 TLS 客户端配置
    pub fn client_config() -> Result<ClientConfig> {
        let mut root_cert_store = RootCertStore::empty();
        
        // 加载系统根证书
        let certs = rustls_native_certs::load_native_certs()
            .context("Failed to load native certificates")?;
            
        for cert in certs {
            root_cert_store
                .add(rustls::pki_types::CertificateDer::from(cert.0))
                .context("Failed to add certificate to root store")?;
        }
        
        let config = ClientConfig::builder()
            .with_root_certificates(root_cert_store)
            .with_no_client_auth();
            
        Ok(config)
    }
    
    /// 从文件创建 TLS 服务器配置
    pub fn server_config_from_files<P: AsRef<Path>>(
        cert_path: P,
        key_path: P,
    ) -> Result<ServerConfig> {
        // 加载证书
        let cert_file = File::open(cert_path)
            .context("Failed to open certificate file")?;
        let mut reader = BufReader::new(cert_file);
        let certs = certs(&mut reader)
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to parse certificates")?;
        
        // 加载私钥
        let key_file = File::open(key_path)
            .context("Failed to open private key file")?;
        let mut reader = BufReader::new(key_file);
        let keys = pkcs8_private_keys(&mut reader)
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to parse private key")?;
        
        if keys.is_empty() {
            anyhow::bail!("No private keys found");
        }
        
        // 使用证书和私钥创建服务器配置
        let config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(
                certs.into_iter().map(rustls::pki_types::CertificateDer::from).collect(),
                rustls::pki_types::PrivateKeyDer::from(keys[0].clone())
            )
            .context("Failed to create server config with certificate and key")?;
            
        Ok(config)
    }
    
    /// 创建自签名证书的 TLS 服务器配置 (用于测试)
    pub fn server_config_with_self_signed() -> Result<ServerConfig> {
        todo!("Generate self-signed certificate for testing")
    }
}