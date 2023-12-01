import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Certificate {
  'id' : bigint,
  'user_id' : bigint,
  'course_id' : bigint,
  'issue_date' : bigint,
}
export interface Course {
  'id' : bigint,
  'title' : string,
  'updated_at' : [] | [bigint],
  'description' : string,
  'created_at' : bigint,
  'lessons' : BigUint64Array | bigint[],
}
export type Error = { 'NotFound' : { 'msg' : string } };
export interface Lesson {
  'id' : bigint,
  'title' : string,
  'updated_at' : [] | [bigint],
  'content' : string,
  'created_at' : bigint,
}
export type Result = { 'Ok' : null } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : Certificate } |
  { 'Err' : Error };
export type Result_2 = { 'Ok' : Course } |
  { 'Err' : Error };
export type Result_3 = { 'Ok' : Lesson } |
  { 'Err' : Error };
export type Result_4 = { 'Ok' : User } |
  { 'Err' : Error };
export type Result_5 = { 'Ok' : boolean } |
  { 'Err' : Error };
export interface User {
  'id' : bigint,
  'username' : string,
  'public_key' : string,
}
export interface _SERVICE {
  'add_course' : ActorMethod<[Course], [] | [Course]>,
  'add_lesson' : ActorMethod<[Lesson, bigint], Result>,
  'delete_course' : ActorMethod<[bigint], Result>,
  'delete_lesson' : ActorMethod<[bigint], Result>,
  'delete_user' : ActorMethod<[bigint], Result>,
  'get_certificate' : ActorMethod<[bigint], Result_1>,
  'get_course' : ActorMethod<[bigint], Result_2>,
  'get_lesson' : ActorMethod<[bigint], Result_3>,
  'get_user' : ActorMethod<[bigint], Result_4>,
  'issue_certificate' : ActorMethod<[bigint, bigint], Result_1>,
  'register_user' : ActorMethod<[string, string], Result_4>,
  'update_course' : ActorMethod<[bigint, string, string], Result_2>,
  'update_lesson' : ActorMethod<[bigint, string, string], Result_3>,
  'verify_certificate' : ActorMethod<[bigint, bigint], Result_5>,
}
