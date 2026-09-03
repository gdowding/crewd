{-# LANGUAGE DataKinds #-}
{-# LANGUAGE TypeOperators #-}

module Main where

import Data.Aeson
import Data.Text
import Data.Time.Calendar
import GHC.Generics
import Network.Wai
import Network.Wai.Handler.Warp
import Servant
import qualified Data.Vector as V
import qualified Data.ByteString.Lazy as BL
import System.Exit as Exit
import Schedule


type ScheduleAPI = "schedule" :> Get '[JSON] [Event]

instance ToJSON Event

server :: [Event] -> Server ScheduleAPI
server = return

scheduleAPI :: Proxy ScheduleAPI
scheduleAPI = Proxy

app :: [Event] -> Application
app events = serve scheduleAPI (server events)

schedulePath = "/Users/gdowding/git/github/gdowding/crewd/schedule.csv"
downloadDirectory = "/Users/gdowding/git/github/gdowding/crewd/results"
raceInfo = "https://race.styc.org/race_info/"

runServer schedData = do
  (_, events) <- readEventCSV schedData
  return $ run 8081 (app (V.toList events))


main :: IO ()
main = do
  csvData <- BL.readFile schedulePath
  case runServer csvData of
    Left err -> Exit.die err
    Right io -> io
