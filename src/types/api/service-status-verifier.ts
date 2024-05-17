import type { Service } from './Service';
import type { ServiceVerificationResult } from './service-verification-result';


export interface ServiceStatusVerifier {
  verify(host: Service): Promise<ServiceVerificationResult| undefined>
}
