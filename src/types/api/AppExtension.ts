import type { ServiceIdentifier } from './ServiceIdentifier';
import type { ServiceStatusVerifier } from './service-status-verifier';
import type { Service } from './Service';
import type { ServiceVerificationResult } from './service-verification-result';


export abstract class AppExtension<T> implements ServiceStatusVerifier {
  abstract identity: string;
  abstract label: string;
  abstract version: string;
  app: T;
  constructor(app: T) {
    this.app = app;
  }
  abstract verify(service: Service): Promise<ServiceVerificationResult | undefined>;
  abstract start(): Promise<void>;
  abstract stop(): Promise<void>;
}
