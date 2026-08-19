module Main (main) where

import System.IO
import Data.Char (isSpace)
import Data.List (intercalate, dropWhileEnd)
import Data.List.Split (splitOn)
import Network.HTTP
import Text.HTML.TagSoup



trimEnd :: String -> String
trimEnd = dropWhileEnd isSpace

-- Replace embedded " with "" and wrap entire string in double quotes.
escapeCSV :: [Char] -> [Char]
escapeCSV s = (wrapDoubleQuote . escapeQuotes) s
  where
    wrapDoubleQuote s' = "\"" ++ s' ++ "\""
    escapeQuotes s'' = concatMap (\c -> if c == '"' then "\"\"" else [c]) s''


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

getRowData row = map (escapeCSV . trimEnd . unwords . (map ((fromTagText)))) $ map (filter isTagText) row

getValuesFromResult :: [Tag [Char]] -> ([String], [[String]])
getValuesFromResult r =
  let
    block = takeWhile (~/= "</table>") r
    rows = partitions (~== "<tr>") block
    th = partitions (~== "<th>") $ rows !! 0
    dataHeadings = map getDataHeadings th
    tdPart = map (partitions (~== "<td>")) $ drop 1 rows
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

getRaceInfo t =
  let
    block = takeWhile (~/= "</td>") $  dropWhile (~/= "<p class=racetitle>") t
    ps = take 3 $ partitions (~== "<p>") block
  in
    map (escapeCSV . trimEnd . innerText) ps

main :: IO ()
main = do
  do
    contents <- readFile "/Users/gdowding/git/github/gdowding/crewd/results/race2.htm"
    let tags = parseTags contents
    -- Get series, date and sponsor for results
    let raceHeadings = ["series", "date", "sponsor"]
    let raceInfo = getRaceInfo tags
    -- Get results for each class
    let resultsSrc = partitions ( ~== "<p class=classtitle>" ) tags
    let results = map getResults resultsSrc
    -- The heading for each class is the same. It should only be incouded in the output once.
    -- So get the class result heading from the first result.
    -- Need to prepend the raceInfo headings
    let h = fst $ head results
    printTable [raceHeadings ++ h]
    -- Results
    let ds = map snd results
    printTable $ map ((++) raceInfo) $ concat ds
    -- let series = trimEnd . innerText $ raceInfo !! 0
    -- let raceDate = trimEnd . innerText $ raceInfo !! 1
    -- let clubName = trimEnd . innerText $ raceInfo !! 3
    -- putStrLn series
    -- putStrLn raceDate
    -- putStrLn clubName
    -- let raceDate = dropWhile
