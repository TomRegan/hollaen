(ns hello.core
  (:gen-class)
  (:use seesaw.core))

(import 'javax.swing.JOptionPane)

(def message "Helllo, World!")

(defn -main [& args]
  "Says hello."
  (invoke-later
   (-> (alert message
              :title "Hello"
              :type :info
              :on-exit :exit)
       show!))
  (println message))
