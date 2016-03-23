(**
 * Copyright 2015 Tom Regan
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *)

let rec fib ?(a=0) ?(b=1) n =
  match n with
  | 0 -> a
  | _ -> fib (n - 1) ~a:b ~b:(a + b)

let main n =
  if n < 0 then failwith "negative number"
  else Printf.printf "%d\n" (fib n)

let () =
  match Sys.argv with
  | [| _; n |] -> main (int_of_string n)
  | _ -> failwith "usage: fibonacci [int]"
