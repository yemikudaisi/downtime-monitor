import { AppExtension, type ExtensibleApp, type Service, type ServiceIdentifier } from "~/types/api";
import { ConnectionString } from 'connection-string';
import type { App } from "~/core/App";
import { ConnectionStringException } from "~/types/api/exceptions/connection-string.excpetion";
import type { ServiceVerificationResult } from "~/types/api/service-verification-result";
import { invoke } from '@tauri-apps/api'

type SmtpConfig = {
  host: string,
  port: number,
  secure: boolean,
  user: string,
  pass: string,
};

export default class SmtpExtension extends AppExtension<App> {
  identity = "icmp-ping";
  label = "ICMP Ping";
  version = "0.0.1";

  /**
   * Requires the service to have the connection string format
   * protocol://user:password@host1:123?secure=1, 
   * @param service The seevice to be checked
   */
  async verify(service: Service): Promise<ServiceVerificationResult | undefined> {
    const connectionObject = new ConnectionString(service.connectionString);
    const hosts = connectionObject.hosts;
    if (!connectionObject.hosts || connectionObject.hosts?.length < 1 || !connectionObject.hosts[0].name || !connectionObject.user || !connectionObject.password) {
      throw new ConnectionStringException(`Connection string must follow the format "protocol://user:password@host1:123?secure=1"`);
    }

    const smtpConfig: SmtpConfig = {
      host: connectionObject.hosts[0].name,
      port: connectionObject.hosts[0].port || 587,
      secure: ((connectionObject.params?.secure)? true: false),
      user: connectionObject.user,
      pass: connectionObject.password
    };

    console.log(smtpConfig);

    const response = await invoke('verify_smtp', { 'smtpConfig': smtpConfig });
    if (response as ServiceVerificationResult !== undefined)
      return response as ServiceVerificationResult
    else return undefined;
  }

  async start(): Promise<void> {
    //const cs = 'smtp://user:password@emailcom:587?secure=1';
    //const cs = 'smtp://mpe9bfhdqyrmiaq:g5mxkuqtf84luw24enqhrh6e9pvl4a@tempmail.us.com:?secure=1';
    const cs = 'smtp.freesmtpservers.com25';
    const res = await this.verify({
      name: 'Service 1',
      connectionString: cs,
      enabled: true,
      serviceIdentifier: '123'
    })
    console.log(res?.message);
    console.log('SMTP Extension started');
  }
  async stop(): Promise<void> {
    console.log('SMTP Extension stopped');
    await new Promise(() => { });
  }
}