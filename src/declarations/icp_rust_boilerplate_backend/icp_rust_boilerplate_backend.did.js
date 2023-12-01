export const idlFactory = ({ IDL }) => {
  const Course = IDL.Record({
    'id' : IDL.Nat64,
    'title' : IDL.Text,
    'updated_at' : IDL.Opt(IDL.Nat64),
    'description' : IDL.Text,
    'created_at' : IDL.Nat64,
    'lessons' : IDL.Vec(IDL.Nat64),
  });
  const Lesson = IDL.Record({
    'id' : IDL.Nat64,
    'title' : IDL.Text,
    'updated_at' : IDL.Opt(IDL.Nat64),
    'content' : IDL.Text,
    'created_at' : IDL.Nat64,
  });
  const Error = IDL.Variant({ 'NotFound' : IDL.Record({ 'msg' : IDL.Text }) });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error });
  const Certificate = IDL.Record({
    'id' : IDL.Nat64,
    'user_id' : IDL.Nat64,
    'course_id' : IDL.Nat64,
    'issue_date' : IDL.Nat64,
  });
  const Result_1 = IDL.Variant({ 'Ok' : Certificate, 'Err' : Error });
  const Result_2 = IDL.Variant({ 'Ok' : Course, 'Err' : Error });
  const Result_3 = IDL.Variant({ 'Ok' : Lesson, 'Err' : Error });
  const User = IDL.Record({
    'id' : IDL.Nat64,
    'username' : IDL.Text,
    'public_key' : IDL.Text,
  });
  const Result_4 = IDL.Variant({ 'Ok' : User, 'Err' : Error });
  const Result_5 = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : Error });
  return IDL.Service({
    'add_course' : IDL.Func([Course], [IDL.Opt(Course)], []),
    'add_lesson' : IDL.Func([Lesson, IDL.Nat64], [Result], []),
    'delete_course' : IDL.Func([IDL.Nat64], [Result], []),
    'delete_lesson' : IDL.Func([IDL.Nat64], [Result], []),
    'delete_user' : IDL.Func([IDL.Nat64], [Result], []),
    'get_certificate' : IDL.Func([IDL.Nat64], [Result_1], ['query']),
    'get_course' : IDL.Func([IDL.Nat64], [Result_2], ['query']),
    'get_lesson' : IDL.Func([IDL.Nat64], [Result_3], ['query']),
    'get_user' : IDL.Func([IDL.Nat64], [Result_4], ['query']),
    'issue_certificate' : IDL.Func([IDL.Nat64, IDL.Nat64], [Result_1], []),
    'register_user' : IDL.Func([IDL.Text, IDL.Text], [Result_4], []),
    'update_course' : IDL.Func([IDL.Nat64, IDL.Text, IDL.Text], [Result_2], []),
    'update_lesson' : IDL.Func([IDL.Nat64, IDL.Text, IDL.Text], [Result_3], []),
    'verify_certificate' : IDL.Func(
        [IDL.Nat64, IDL.Nat64],
        [Result_5],
        ['query'],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
