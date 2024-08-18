use anyhow::Result;
use serde_json::{ json, Value };

#[tokio::test]
async fn test_simple_base() -> httpc_test::Result<()> {
    // Create a new httpc test client with a base URL (will be prefixed for all calls)
    // The client will have a cookie_store.
    let hc = httpc_test::new_client("http://localhost:3000")?;

    // Simple do_get
    let res = hc.do_get("/get_message/first_queue").await?; // httpc_test::Response
    let status = res.status();
    // Pretty print the result (status, headers, response cookies, client cookies, body)
    res.print().await?;

    // Another post (with the cookie store updated from the login request above )
    let res = hc.do_post(
        "/insert_message/first_queue",
        json!({
				"message": "message for first queue"
			})
    ).await?;
    res.print().await?;

    // Simple do_get
    let res = hc.do_get("/get_message/first_queue").await?; // httpc_test::Response
    let status = res.status();
    // Pretty print the result (status, headers, response cookies, client cookies, body)
    res.print().await?;

    // Same woth do_patch, do_put.

    Ok(())
}
