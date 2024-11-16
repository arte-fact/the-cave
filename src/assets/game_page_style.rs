pub const STYLE: &str = r#"
  html {
    font-weight: 400;
    font-size: 18px;
    color: gray;
    background: black;
    overflow: hidden;
    
    .stats {
      padding: 9px 18px;
    }
    .game {
      display: flex;
      flex-wrap: wrap;
      justify-content: center;
      align-items: center;
      height: 100vh;

      
      .events, .stats {
        position: fixed;
        top: 0;
        right: 0;
        padding: 18px;
        bottom: 0;
      }

      .events {
        flex-direction: column;
        gap: 9px;
      }

      .map {
        transform: matrix3d(1, 0, 0, 0, 0, 1, 0, -0.0005, 0, 0, 1, 0, 0, 0, 0, 1);
      }
    }
    .row {
        position: relative;
        display: flex;
        .char {
          position: relative;
      }
    }

    .tile {
    --size: 32px;
      font-size: var(--size);
      width: var(--size);
      height: var(--size);
      max-width: var(--size);
      max-height: var(--size);
      overflow: visible;
      text-align: center;
      line-height: 3px;
    }
  }
"#; 

