import type { Service } from './Service';


export interface ServiceStatusChecker {
  check(host: Service): Promise<boolean>;
}
