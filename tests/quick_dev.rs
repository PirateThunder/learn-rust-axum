use anyhow::Result;

// HI
#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8000")?;

    hc.do_get("/test").await?.print().await?;

    Ok(())
}