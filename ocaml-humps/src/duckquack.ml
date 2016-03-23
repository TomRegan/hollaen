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

open Unix

let handle_request socket address =
  match address with
  | ADDR_INET(host, port) ->
     Printf.printf "RX (%s:%d) ...\n%!"
                   (string_of_inet_addr host)
                   port
  | _ -> ();;


let rec serve socket =
  match accept socket with
  | peer_socket, peer_address ->
     handle_request peer_socket peer_address;
     serve socket;;


let initialise_socket host port =
  let socket = socket PF_INET SOCK_STREAM 0 in
  Printf.printf "Listening on %s:%d\n%!"
                (string_of_inet_addr host)
                port;
  setsockopt socket SO_REUSEADDR true;
  let address = ADDR_INET (host, port) in
  bind socket address;
  listen socket 0;
  socket


let main =
  match initialise_socket inet_addr_loopback 8000 with
  | socket ->
     serve socket;;


let () =
  main
