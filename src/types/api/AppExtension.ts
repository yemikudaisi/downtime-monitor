import type { ServiceIdentifier } from './ServiceIdentifier';
import type { ServiceStatusChecker } from './ServiceStatusChecker';
import type { Service } from './Service';


export abstract class AppExtension<T> implements ServiceStatusChecker {
  abstract identity: string;
  abstract label: string;
  abstract version: string;
  app: T;
  constructor(app: T) {
    this.app = app;
  }
  abstract check(service: Service): Promise<boolean>;
  abstract start(): Promise<void>;
  abstract stop(): Promise<void>;
}
