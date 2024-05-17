import SmtpExtension from "~/extensions/smtp";
import { ExtensionManager, type ExtensibleApp } from "../types/api";

export class App implements ExtensibleApp {
  extensionManager = new ExtensionManager<App>();
  constructor() {}
  start() {
    this.extensionManager.loadDir(this, [new SmtpExtension(this)])
  }
  stop() {
    this.extensionManager.unloadAll();
  }
}