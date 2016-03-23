(defproject clods "0.1.0-SNAPSHOT"
  :description "FIXME: write description"
  :url "http://github.com/TomRegan/clods.git"
  :main clods.core
  :aot [clods.core]
  :target-path "target/clods"
  :license {:name "Apache 2.0"
            :url "http://www.apache.org/licenses/LICENSE-2.0.txt"}
  :dependencies [[org.clojure/clojure "1.6.0"]
                 [clj-http "1.1.2"]])
