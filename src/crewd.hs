{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE DeriveGeneric #-}

module Main (main) where

import qualified Data.Text as T
import Data.Time.Calendar (Day)
import GHC.Generics (Generic)
import Data.Csv
import qualified Data.ByteString as BS
import qualified Data.ByteString.Char8 as BC
import Data.Time.Format (defaultTimeLocale,  parseTimeM)
import Text.HTML.TagSoup
import System.Directory (createDirectoryIfMissing)
import System.FilePath ((</>), (-<.>), dropDrive, takeFileName, takeDirectory)
import Data.List (intercalate)
import qualified Data.Text.IO as TIO
import Network.HTTP.Simple
import Data.Time.Calendar (toGregorian)
import Data.List.Split (splitOn)
import Data.Char (isSpace)
import Data.List (dropWhileEnd)
import qualified Network.HTTP.Client as CL
import Data.Time.Clock (getCurrentTime, utctDay)
import qualified Data.Vector as V
import qualified Data.ByteString.Lazy as BL
import System.Exit as Exit


schedulePath = "/Users/gdowding/git/github/gdowding/crewd/schedule.csv"
downloadDirectory = "/Users/gdowding/git/github/gdowding/crewd/results"
raceInfo = "https://race.styc.org/race_info/"

data Event = Event
  { regatta    :: !T.Text
  , series     :: !T.Text
  , start_date :: !Day
  , race       :: !T.Text
  } deriving (Show, Generic)

instance FromNamedRecord Event

parseDate :: BS.ByteString -> Parser Day
parseDate s =
  case parseTimeM True defaultTimeLocale "%Y-%m-%d" (BC.unpack s) of
    Just day -> pure day
    Nothing -> fail $ "Could not parse ISO-8601 date: " ++ BC.unpack s

instance FromField Day where
  parseField = parseDate

processResult event =
  do
    let url = getEvtUrl event
    req <- parseRequest url
    let filePath = localPath req
    -- Is this a race condition if multiple processes are attempting to create directory at same time?
    -- or is it thread safe?
    createDirectoryIfMissing True $ takeDirectory filePath
    -- respBody <- fetchResult req filePath
    -- let tags = parseTags $ T.unpack (TE.decodeUtf8 respBody)
    respBody <- TIO.readFile filePath
    let tags = parseTags $ T.unpack  respBody
    let classes = partitions (~== ("<p class=classtitle>" :: String)) tags
    let results = flattenSnd $ map getResults classes
    let resultString = intercalate "\n" $ map rowToCsv results
    let csvPath = filePath -<.> ".csv"
    putStrLn filePath
    putStrLn csvPath
    -- putStrLn resultString
    TIO.writeFile csvPath $ T.pack resultString

flattenSnd results =
  let
    (headList, _):_ = results
    resultData = concatMap snd results
  in
    headList:resultData

getEvtUrl :: Event -> String
getEvtUrl Event {regatta = r, series = s, start_date = d, race = n} =
  raceInfo ++ regatta' ++ "/" ++ series' ++ "/" ++ year ++ "/" ++ "race" ++ race' ++ ".htm"
  where
    regatta' = map (\x -> if x == ' ' then '_' else x) $ T.unpack r
    (y, _, _) = toGregorian d
    year = show y
    series'  = "Series" ++ T.unpack s
    race' = T.unpack n

getResults :: [Tag String] -> ([String], [[String]])
getResults r =
  let (cHead, cData) = getHeadFromResult r
      (dHead, dValues) = getValuesFromResult r
      classDataValues = map (\d -> cData ++ d) dValues
  in
    (cHead ++ dHead, classDataValues)

classFromSect s = unwords . words . innerText . drop 1  $ take 2 s

-- get heading data from single class result
getHeadFromResult r =
     let classTitle = classFromSect r
         -- split into fields and remove separators
         classFields = splitOn " - " classTitle
         fields = map (\h -> (fst h, dropWhile (\c -> c == ':' || c == ' ') $ snd h)) $ map (break (== ':')) classFields
         -- transpose from [(field1, data1) ...] to [[field1, ...], [data1, ...]
         headings =  map (escapeCSV . fst) fields
         values = map (escapeCSV . snd) fields
         -- clean up the second column by adding a field name and using the supplied field name as data.
         description = headings !! 1
         h = (\(x:_:xs) -> x : "Description" : xs) headings
         d = (\(x:_:xs) -> x : description : xs) values
     in
       (h, d)

getRowData row = map (escapeCSV . trimEnd . unwords . (map fromTagText)) $ map (filter isTagText) row

getDataHeadings th = escapeCSV . unwords $ filter (not . null) $ map (filter (not . isSpace)) $ map fromTagText $ filter isTagText th

getValuesFromResult :: [Tag [Char]] -> ([String], [[String]])
getValuesFromResult r =
  let
    block = takeWhile (~/= ("</table>" :: String)) r
    rows = partitions (~== ("<tr>" :: String)) block
    th = partitions (~== ("<th>" :: String)) $ head rows
    dataHeadings = map getDataHeadings th
    tdPart = map (partitions (~== ("<td>" :: String))) $ drop 1 rows
    dataValues = map getRowData tdPart
  in
    (dataHeadings, dataValues)


trimEnd :: String -> String
trimEnd = dropWhileEnd isSpace

rowToCsv :: [String] -> String
rowToCsv = intercalate ","

escapeCSV :: [Char] -> [Char]
escapeCSV = wrapDoubleQuote . escapeQuotes
  where
    wrapDoubleQuote s' = "\"" ++ s' ++ "\""
    escapeQuotes = concatMap (\c -> if c == '"' then "\"\"" else [c])

localPath req =
  downloadDirectory </> filePath
  where
    reqHost = BC.unpack $ CL.host req
    reqPath = BC.unpack $ CL.path req
    filePath = reqHost </> dropDrive reqPath

-- Only process events that have already occoured.
runPipeline :: V.Vector Event -> IO ()
runPipeline events = do
  currentDay <- utctDay <$> getCurrentTime
  let pastEvents = V.filter (\e -> start_date e < currentDay) events
  V.forM_ pastEvents processResult
  putStrLn "finish"

main :: IO ()
main = do
  do
    csvData <- BL.readFile schedulePath
    case decodeByName csvData of
      Left err -> Exit.die err
      Right(_, v) -> runPipeline (v :: V.Vector Event)
