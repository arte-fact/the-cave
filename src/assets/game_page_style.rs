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
      flex-direction: column;

      .events {
        flex-direction: column;
        gap: 9px;
      }

      .map {
        // transform: matrix3d(1, 0, 0, 0, 0, 1, 0, -0.0005, 0, 0, 1, 0, 0, 0, 0, 1);
        margin-top: 4em;
      }
    }
    .row {
        position: relative;
        display: flex;
        .char {
          position: relative;
      }
    }

    .controls {
      display: flex;
      justify-content: center;
      align-items: center;
      width: 100%;
      gap: 9px;
      margin-top: 9px;
      button {
        font-size: 64px;
        background: none;
        border: none;
        cursor: pointer;

        text-decoration: none;
        color: gray;
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

