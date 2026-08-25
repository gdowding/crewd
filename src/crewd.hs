{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE DeriveGeneric #-}

module Main (main) where

import System.IO
import System.Exit as Exit
import qualified Data.ByteString as BS
import qualified Data.ByteString.Char8 as BC
import qualified Data.ByteString.Lazy as BL
import Data.Char (isSpace)
import Data.Csv
import Data.List (intercalate, dropWhileEnd)
import Data.List.Split (splitOn)
import Data.Time.Clock (getCurrentTime, utctDay)
import qualified Data.Text as T
import qualified Data.Text.Encoding as TE
import qualified Data.Text.IO as TIO
import Data.Time.Calendar (Day, toGregorian)
import Data.Time.Format (defaultTimeLocale,  parseTimeM)
import qualified Data.Vector as V
import GHC.Generics (Generic)
import Network.HTTP.Simple
import qualified Network.HTTP.Client as CL
import System.Directory (createDirectoryIfMissing)
import System.FilePath ((</>), (-<.>), dropDrive, takeFileName, takeDirectory)

import Text.HTML.TagSoup


data Event = Event
  { regatta    :: !T.Text
  , series     :: !T.Text
  , start_date :: !Day
  , race       :: !T.Text
  } deriving (Show, Generic)

parseDate :: BS.ByteString -> Parser Day
parseDate s =
  case parseTimeM True defaultTimeLocale "%Y-%m-%d" (BC.unpack s) of
    Just day -> pure day
    Nothing -> fail $ "Could not parse ISO-8601 date: " ++ BC.unpack s

instance FromField Day where
  parseField = parseDate


instance FromNamedRecord Event


trimEnd :: String -> String
trimEnd = dropWhileEnd isSpace

-- Replace embedded " with "" and wrap entire string in double quotes.
escapeCSV :: [Char] -> [Char]
escapeCSV = wrapDoubleQuote . escapeQuotes
  where
    wrapDoubleQuote s' = "\"" ++ s' ++ "\""
    escapeQuotes = concatMap (\c -> if c == '"' then "\"\"" else [c])


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


getDataHeadings th = escapeCSV . unwords $ filter (not . null) $ map (filter (not . isSpace)) $ map fromTagText $ filter isTagText th

-- Similar to getRowData but this only applies to a single string value.
getText t = trimEnd . unwords $ map fromTagText $ filter isTagText t

getRowData row = map (escapeCSV . trimEnd . unwords . (map fromTagText)) $ map (filter isTagText) row

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


-- get header and results for a single class

getResults :: [Tag String] -> ([String], [[String]])
getResults r =
  let (cHead, cData) = getHeadFromResult r
      (dHead, dValues) = getValuesFromResult r
      classDataValues = map (\d -> cData ++ d) dValues
  in
    (cHead ++ dHead, classDataValues)


printTable :: [[String]] -> IO ()
printTable table = putStr (unlines (map (intercalate ",") table))

rowToCsv :: [String] -> String
rowToCsv = intercalate ","


getRaceInfo :: [Tag String] -> [[Char]]
getRaceInfo t =
  let
    block = takeWhile (~/= ("</td>" :: String)) $  dropWhile (~/= ("<p class=racetitle>" :: String)) t
    ps = take 3 $ partitions (~== ("<p>" :: String)) block
  in
    map (escapeCSV . trimEnd . innerText) ps


-- getUrl :: Text -> Text -> Day -> Text -> String
-- getUrl regatta series date race =
-- Currently hardcoded to Ballard Cup events only

getEvtUrl :: Event -> String
getEvtUrl Event {regatta = r, series = s, start_date = d, race =n} =
  raceInfo ++ regatta' ++ "/" ++ series' ++ "/" ++ year ++ "/" ++ "race" ++ race' ++ ".htm"
  where
    raceInfo = "https://race.styc.org/race_info/"
    regatta' = map (\x -> if x == ' ' then '_' else x) $ T.unpack r
    (y, _, _) = toGregorian d
    year = show y
    series'  = "Series" ++ T.unpack s
    race' = T.unpack n

------------------------------------------------------------
-- Manage download directory.

downloadDirectory = "/Users/gdowding/git/github/gdowding/crewd/results"

fullPath req =
  downloadDirectory </> dirPath
  where
    reqHost = BC.unpack $ CL.host req
    reqPath = BC.unpack $ CL.path req
    dirPath = reqHost </> dropDrive (takeDirectory reqPath)


localPath req =
  downloadDirectory </> filePath
  where
    reqHost = BC.unpack $ CL.host req
    reqPath = BC.unpack $ CL.path req
    filePath = reqHost </> dropDrive reqPath


------------------------------------------------------------
-- fetch result. Save result to filePath and return body of response.

fetchResult req filePath =
  do
    response <- httpBS req
    let bodyContent = getResponseBody response
    BS.writeFile filePath bodyContent
    return bodyContent

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


------------------------------------------------------------
-- Pipeline

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
    let schedulePath = "/Users/gdowding/git/github/gdowding/crewd/schedule.csv"
    csvData <- BL.readFile schedulePath
    case decodeByName csvData of
      Left err -> Exit.die err
      Right(_, v) -> runPipeline (v :: V.Vector Event)
