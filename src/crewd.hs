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
import Data.Text (Text, pack, unpack)
import Data.Time.Calendar (Day, toGregorian)
import Data.Time.Format (defaultTimeLocale,  parseTimeM)
import qualified Data.Vector as V
import GHC.Generics (Generic)
import Network.HTTP
import Text.HTML.TagSoup


data Event = Event
  { regatta    :: !Text
  , series     :: !Text
  , start_date :: !Day
  , race       :: !Text
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
    (dataHeadings, dataValues)  -- (dataHeadings, map getRowData tdPart)


getResults r =
  let (cHead, cData) = getHeadFromResult r
      (dHead, dValues) = getValuesFromResult r
      classDataValues = map (\d -> cData ++ d) dValues
  in
    (cHead ++ dHead, classDataValues)

printTable table = putStr (unlines (map (intercalate ",") table))

getRaceInfo :: [Tag String] -> [[Char]]
getRaceInfo t =
  let
    block = takeWhile (~/= ("</td>" :: String)) $  dropWhile (~/= ("<p class=racetitle>" :: String)) t
    ps = take 3 $ partitions (~== ("<p>" :: String)) block
  in
    map (escapeCSV . trimEnd . innerText) ps


-- getUrl :: Text -> Text -> Day -> Text -> String
-- getUrl regatta series date race =

getEvtUrl :: Event -> String
getEvtUrl Event {regatta = r, series = s, start_date = d, race =n} =
  raceInfo ++ regatta' ++ "/" ++ series' ++ "/" ++ year ++ "/" ++ "race" ++ race' ++ ".htm"
  where
    raceInfo = "https://race.styc.org/race_info/"
    regatta' = map (\x -> if x == ' ' then '_' else x) $ unpack r
    (y, _, _) = toGregorian d
    year = show y
    series'  = "Series" ++ unpack s
    race' = unpack n

printUrls :: String -> IO ()
printUrls = putStrLn



-- TODO: convert event to url


getEvtUrl' Event { regatta = "Ballard Cup" } =
  "BallardCup"


getEvtUrl' Event { regatta = "Foo" } =
  "Some Other Regatta"


-- Only process events that have already occoured.
runPipeline :: V.Vector Event -> IO ()
runPipeline events = do
  currentDay <- utctDay <$> getCurrentTime
  let urls = V.map getEvtUrl $ V.filter (\e -> start_date e < currentDay) events
  V.forM_ urls print

main :: IO ()
main = do
  do
    let schedulePath = "/Users/gdowding/git/github/gdowding/crewd/schedule.csv"
    csvData <- BL.readFile schedulePath
    case decodeByName csvData of
      Left err -> Exit.die err
      Right(_, v) -> runPipeline (v :: V.Vector Event)

    -- putStrLn $ getUrl "Ballard Cup" "III" 2026 "1"

    -- contents <- readFile "/Users/gdowding/git/github/gdowding/crewd/results/race2.htm"
    -- let tags = parseTags contents
    -- -- Get series, date and sponsor for results
    -- let raceHeadings = ["series", "date", "sponsor"]
    -- let raceInfo = getRaceInfo tags
    -- -- Get results for each class
    -- let resultsSrc = partitions ( ~== "<p class=classtitle>" ) tags
    -- let results = map getResults resultsSrc
    -- -- The heading for each class is the same. It should only be incouded in the output once.
    -- -- So get the class result heading from the first result.
    -- -- Need to prepend the raceInfo headings
    -- let h = fst $ head results
    -- printTable [raceHeadings ++ h]
    -- -- Results
    -- let ds = map snd results
    -- printTable $ map ((++) raceInfo) $ concat ds
    -- -- let series = trimEnd . innerText $ raceInfo !! 0
    -- -- let raceDate = trimEnd . innerText $ raceInfo !! 1
    -- -- let clubName = trimEnd . innerText $ raceInfo !! 3
    -- -- putStrLn series
    -- -- putStrLn raceDate
    -- -- putStrLn clubName
    -- -- let raceDate = dropWhile
