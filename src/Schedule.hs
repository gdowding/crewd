{-# LANGUAGE DeriveGeneric #-}

module Schedule (Event(..), readEventCSV) where

import qualified Data.Text as T
import Data.Time.Calendar (Day)
import GHC.Generics (Generic)
import Data.Csv
import Data.Time.Format (defaultTimeLocale,  parseTimeM)
import qualified Data.ByteString as BS
import qualified Data.ByteString.Char8 as BC
import qualified Data.ByteString.Lazy as BL
import qualified Data.Vector as V

data Event = Event
  { regatta    :: !T.Text
  , series     :: !T.Text
  , start_date :: !Day
  , race       :: !T.Text
  } deriving (Show, Generic, Eq)

instance FromNamedRecord Event

parseDate :: BS.ByteString -> Parser Day
parseDate s =
  case parseTimeM True defaultTimeLocale "%Y-%m-%d" (BC.unpack s) of
    Just day -> pure day
    Nothing -> fail $ "Could not parse ISO-8601 date: " ++ BC.unpack s

instance FromField Day where
  parseField = parseDate

-- readEventCSV :: BL.ByteString -> IO (Either String (V.Vector Event))
readEventCSV csvData = decodeByName csvData
