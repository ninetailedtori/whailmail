import { join } from "path";

import { electronApp, is, optimizer } from "@electron-toolkit/utils";
import icon from "@resources/icon.png?asset";
import { app, BrowserWindow, ipcMain, screen, shell } from "electron";

function createWindow(): void {
  const primaryDisplay = screen.getPrimaryDisplay();
  const { width: screenWidth, height: screenHeight } =
    primaryDisplay.workAreaSize;

  const windowWidth = Math.floor(screenWidth / 2);
  const windowHeight = Math.floor(screenHeight / 2);

  const mainWindow = new BrowserWindow({
    width: windowWidth,
    height: windowHeight,
    x: Math.floor(screenWidth / 4),
    y: Math.floor(screenHeight / 4),
    show: false,
    autoHideMenuBar: true,
    alwaysOnTop: true,
    ...(process.platform === "linux" ? { icon } : {}),
    webPreferences: {
      preload: join(__dirname, "../preload/index.js"),
      sandbox: false,
    },
  });

  mainWindow.webContents.session.webRequest.onHeadersReceived(
    (details, callback) => {
      callback({
        responseHeaders: {
          ...details.responseHeaders,
          "Content-Security-Policy": ["script-src 'self' 'unsafe-inline'"],
        },
      });
    }
  );

  mainWindow.on("ready-to-show", () => {
    mainWindow.show();
  });

  mainWindow.webContents.setWindowOpenHandler((details) => {
    shell.openExternal(details.url).catch((err) => {
      console.error("Failed to open external URL:", err);
    });
    return { action: "deny" };
  });

  if (is.dev && process.env["ELECTRON_RENDERER_URL"]) {
    mainWindow.loadURL(process.env["ELECTRON_RENDERER_URL"]).catch((err) => {
      console.error("Failed to load dev server:", err);
    });
  } else {
    mainWindow
      .loadFile(join(__dirname, "../renderer/index.html"))
      .catch((err) => {
        console.error("Failed to load file:", err);
      });
  }
}

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
app
  .whenReady()
  .then((): void => {
    // Set app user model id for windows
    electronApp.setAppUserModelId("com.electron");

    app.setName("WhailMail");

    // Default open or close DevTools by F12 in development
    // and ignore CommandOrControl + R in production.
    // see https://github.com/alex8088/electron-toolkit/tree/master/packages/utils
    app.on("browser-window-created", (_, window) => {
      optimizer.watchWindowShortcuts(window);
    });

    // IPC test
    ipcMain.on("ping", () => console.log("pong"));

    createWindow();

    app.on("activate", function () {
      // On macOS it's common to re-create a window in the app when the
      // dock icon is clicked and there are no other windows open.
      if (BrowserWindow.getAllWindows().length === 0) createWindow();
    });
  })
  .catch((err): void => {
    console.error("Failed to initialize app:", err);
  });

// Quit when all windows are closed, except on macOS. There, it's common
// for applications and their menu bar to stay active until the user quits
// explicitly with Cmd + Q.
app.on("window-all-closed", () => {
  if (process.platform !== "darwin") {
    app.quit();
  }
});

ipcMain.handle("get-platform", () => {
  return process.platform; // "win32", "darwin", "linux"
});

// In this file you can include the rest of your app's specific main process
// code. You can also put them in separate files and require them here.
