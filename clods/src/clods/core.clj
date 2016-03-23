;;; clods.core

;; Copyright 2015 Tom Regan
;;
;; Licensed under the Apache License, Version 2.0 (the "License");
;; you may not use this file except in compliance with the License.
;; You may obtain a copy of the License at
;;
;;     http://www.apache.org/licenses/LICENSE-2.0
;;
;; Unless required by applicable law or agreed to in writing, software
;; distributed under the License is distributed on an "AS IS" BASIS,
;; WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
;; See the License for the specific language governing permissions and
;; limitations under the License.

;;; Commentary:

;; The lein of best fit...

;;; Code:

(ns clods.core
  (:gen-class)
  (require [clods.fizzbuzz :refer [fizzbuzz]]
           [clods.bonjour :refer [bonjour]]
           [clods.http :refer [request]]))


(defn -main
  [& args]
  (cond (some #{"bonjour"} *command-line-args*) (bonjour)
        (some #{"fizzbuzz"} *command-line-args*) (fizzbuzz)
        (some #{"request"} *command-line-args*) (request "http://www.example.com/")
        :else (println "command" *command-line-args* "doesn't exist.")))


;;; clods.core ends here
