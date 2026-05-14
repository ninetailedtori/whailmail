/*
 * SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
 * SPDX-FileContributor: WhailMail contributors
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

import { join } from "path";

import { electronApp, is, optimizer } from "@electron-toolkit/utils";
import icon from "@resources/icon.png?asset";
import { app, BrowserWindow, ipcMain, screen, shell } from "electron";

let mainWindow: BrowserWindow | null = null;

function createWindow(): void {
  const primaryDisplay = screen.getPrimaryDisplay();
  const { width: screenWidth, height: screenHeight } =
    primaryDisplay.workAreaSize;

  const windowWidth = Math.floor(screenWidth / 2);
  const windowHeight = Math.floor(screenHeight / 2);

  mainWindow = new BrowserWindow({
    width: windowWidth,
    height: windowHeight,
    x: Math.floor(screenWidth / 4),
    y: Math.floor(screenHeight / 4),

    show: false,

    titleBarStyle: "hidden",

    ...(process.platform === "linux" ? { icon } : {}),

    webPreferences: {
      preload: join(__dirname, "../preload/index.js"),
      sandbox: false,
      contextIsolation: true,
      nodeIntegration: false,
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
    mainWindow?.show();
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

// ==============================
// App Ready
// ==============================

app
  .whenReady()
  .then((): void => {
    electronApp.setAppUserModelId("com.electron");

    app.setName("WhailMail");

    app.on("browser-window-created", (_, window) => {
      optimizer.watchWindowShortcuts(window);
    });

    // ==============================
    // Window Controls IPC
    // ==============================

    ipcMain.on("window:minimize", () => {
      mainWindow?.minimize();
    });

    ipcMain.on("window:maximize", () => {
      if (!mainWindow) return;

      if (mainWindow.isMaximized()) {
        mainWindow.unmaximize();
      } else {
        mainWindow.maximize();
      }
    });

    ipcMain.on("window:close", () => {
      mainWindow?.close();
    });

    ipcMain.handle("window:isMaximized", () => {
      return mainWindow?.isMaximized() ?? false;
    });

    // Existing IPC
    ipcMain.on("ping", () => console.log("pong"));

    ipcMain.handle("get-platform", () => {
      return process.platform;
    });

    createWindow();

    app.on("activate", function () {
      if (BrowserWindow.getAllWindows().length === 0) {
        createWindow();
      }
    });
  })
  .catch((err): void => {
    console.error("Failed to initialize app:", err);
  });

// ==============================
// Quit Handling
// ==============================

app.on("window-all-closed", () => {
  if (process.platform !== "darwin") {
    app.quit();
  }
});
