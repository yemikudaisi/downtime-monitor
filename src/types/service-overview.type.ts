import { z } from 'zod';
import { ServiceStatus } from './service-status.type'

export const ServiceOverviewSchema = z.object( {
  name: z.string().min(1),
  host: z.string().url(),
  status: z.nativeEnum(ServiceStatus),
  statusDuration: z.string().datetime(),
  lastChecked: z.string().datetime(),
});

export class ServiceOverview {};
export interface ServiceOverview extends z.infer<typeof ServiceOverviewSchema> {}
