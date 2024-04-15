import { ExtensionManager, type ExtensibleApp } from "../types/api";

export class App implements ExtensibleApp {
  extensionManager = new ExtensionManager<App>();
  constructor() {}
  start() {
    this.extensionManager.loadDir(this, 'extensions')
  }
  stop() {
    this.extensionManager.unloadAll();
  }
}