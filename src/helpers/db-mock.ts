import { ServiceStatus } from "~/types";

export const db = {
  services: {
    overview: [
      {
        name: "Service 1",
        host: "192.3.4.14:3000",
        status: ServiceStatus.Online,
        statusDuration: "2024-01-01T00:00:00Z",
        lastChecked: "2024-04-14T13:08:31Z",

      },
      {
        name: "Service 2",
        host: "192.3.4.14:3000",
        status: ServiceStatus.Online,
        statusDuration: "2024-01-01T00:00:00Z",
        lastChecked: "2024-04-14T13:08:31Z",

      },
      {
        name: "Service 5",
        host: "192.3.4.14:3000",
        status: ServiceStatus.Online,
        statusDuration: "2024-01-01T00:00:00Z",
        lastChecked: "2024-04-14T13:08:31Z",

      },
      {
        name: "NA Web SMTP",
        host: "192.3.4.14:3000",
        status: ServiceStatus.Offline,
        statusDuration: "2024-01-01T00:00:00Z",
        lastChecked: "2024-04-14T13:08:31Z",

      },
      {
        name: "Service 6",
        host: "192.3.4.14:3000",
        status: ServiceStatus.Offline,
        statusDuration: "2024-01-01T00:00:00Z",
        lastChecked: "2024-04-14T13:08:31Z",
      }
    ]
  }

}