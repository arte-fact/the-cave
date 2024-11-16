pub const SCRIPT: &str = r#"
  let is_fetching = false;
  let odl_time = 0;
  document.addEventListener('keydown', async function (e) {
    if (!["z", "q", "s", "d", "h", "j", "k", "l", "ArrowLeft", "ArrowRight", "ArrowUp", "ArrowDown", "Enter"].includes(e.key)) {
      return;
    }
    if (Date.now() - odl_time < 100) {
      return;
    }
    odl_time = Date.now();
    if (is_fetching) {
      return;
    }
    is_fetching = true;

    let response = await fetch(
      `/key/${e.key}`,
      {
        method: 'GET',
        headers: {},
      });
    let data = await response.text();
    document.getElementById('content').innerHTML = data;
    is_fetching = false;
  });
"#;
