import Data.Char
import Data.Function
import Data.List

encrypt :: String -> [String]
encrypt line =
  let trimmed = filter isAlpha line
      l = length trimmed
      root = (sqrt $ fromInteger $ toInteger l) :: Double
      low = floor root
      high = ceiling root
      gridCandidates = [(low, low), (low, high), (high, high)]
      gridSize =
        minimumBy (compare `on` uncurry (*)) $
        filter (\t -> uncurry (*) t >= l) gridCandidates
      columns = snd gridSize
      grid = makeGrid trimmed columns
   in map (slice grid) [0 .. columns - 1]

slice :: [String] -> Int -> String
slice ss i =
  filter isAlpha $
  map
    (\s ->
       if i < length s
         then s !! i
         else ' ')
    ss

makeGrid :: String -> Int -> [String]
makeGrid l c =
  if length l < c
    then [l]
    else take c l : makeGrid (drop c l) c

main :: IO ()
main = interact $ unwords . encrypt
