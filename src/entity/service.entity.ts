import { Column, Entity } from "typeorm";
import { AbstractEntity } from "./abstact.entity";
import { nullable } from "zod";

@Entity('services')
export class Service extends AbstractEntity {
  @Column({nullable: false})
  name!: string;

  @Column({nullable: false})
  connectionString!: string;

  @Column({nullable: false})
  enabled!: boolean;

  @Column({nullable: false})
  serviceIdentifier!: string;
}