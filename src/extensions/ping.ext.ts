import { AppExtension, type ExtensibleApp, type Service, type ServiceIdentifier } from "~/types/api";
import type { ServiceVerificationResult } from "~/types/api/service-verification-result";

export default class PingExtension extends AppExtension<ExtensibleApp> {
  identity = "icmp-ping";
  label =  "ICMP Ping";
  version = "0.0.1";

  verify(host: Service): Promise<ServiceVerificationResult> {
    throw new Error("Method not implemented.");
  }
  async start(): Promise<void> {
   console.log('ICMP Ping extension started');
   await new Promise(() => {});
  }
  async stop(): Promise<void> {
    console.log('ICMP Ping extension stopped');
   await new Promise(() => {});
  }

}