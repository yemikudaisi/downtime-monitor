import { readDir, BaseDirectory } from '@tauri-apps/api/fs';
import * as path from '@tauri-apps/api/path';
import * as http from "@tauri-apps/api/http"
import { AppExtension } from '../types/api/AppExtension';
import { convertFileSrc } from '@tauri-apps/api/tauri';


export class ExtensionManager<T> {
  private _extensions: AppExtension<T>[] = [];
  async load(extension: AppExtension<T>) {
    this._extensions.push(extension);
    await extension.start();
  }
  async unload(extension: AppExtension<T>) {
    let target = this._extensions.find(e => e.identity === extension.identity);
    if (!target) return false;
    await target.stop();
    return true;
  }

  async unloadAll() {
    for (let extension of this._extensions) {
      await extension.stop();
    }
  }

  async loadDir(app: T, _source: string | AppExtension<T>[]) {
    if(typeof _source === 'string'){
      const files = await readDir(_source, { dir: BaseDirectory.AppData, recursive: true });
    console.log('>>>>>>>>>>>>>>>>>>loading dir <<<<<<<<<<<<<<<');
    console.log(files)
    for (let file of files) {
      const response = await http.fetch(convertFileSrc(file.path))
      const jsCode = await response.data as string;
      let Extension = await import(/* @vite-ignore */`data:text/javascript;base64,${btoa(jsCode)}`);
      //let Extension = await import(convertFileSrc(file.path));
      let plugin = new Extension(app);
      await this.load(Extension as AppExtension<T>);
    }
    } else {
      for (let ext of _source ){
        await this.load(ext);
      }
    }
  }
}
