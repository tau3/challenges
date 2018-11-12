{-# LANGUAGE FlexibleContexts, ScopedTypeVariables #-}

import Control.Monad.ST
import Data.Foldable
import Data.Maybe
import Data.STRef

type Point = (Int, Int)

data Cell a
  = Empty
  | Bomb a
  deriving (Eq, Show)

type Grid = [(Point, Cell Int)]

makeCell :: Int -> Int -> [String] -> (Point, Cell Int)
makeCell row column rawData = ((row, column), state)
  where
    state =
      if (rawData !! row) !! column == '.'
        then Empty
        else Bomb 3

parseGrid :: [String] -> Int -> Int -> Grid
parseGrid rawData r c =
  runST $ do
    res <- newSTRef []
    for_ [0 .. r - 1] $ \ri ->
      for_ [0 .. c - 1] $ \ci -> do
        let cell = makeCell ri ci rawData
        modifySTRef' res (\l -> cell : l)
    readSTRef res

plantBombs :: Grid -> Grid
plantBombs =
  map
    (\(p, c) ->
       if c == Empty
         then (p, Bomb 3)
         else (p, c))

set :: Grid -> Point -> Cell Int -> Grid
set g p v = (p, v) : filter (\(p', _) -> p' /= p) g

setMultiple :: Grid -> [Point] -> Cell Int -> Grid
setMultiple g [] _ = g
setMultiple g (p:ps) v = setMultiple (set g p v) ps v

destroyed :: Grid -> Point -> [Point]
destroyed grid (r, c) = (r, c) : neighbours (r, c)
  where
    neighbours p' = filter fits [upper p', lower p', left p', right p']
    upper (r', c') = (r' - 1, c')
    lower (r', c') = (r' + 1, c')
    left (r', c') = (r', c' - 1)
    right (r', c') = (r', c' + 1)
    fits (r', c') =
      r' >= 0 && c' >= 0 && r' < countRows grid && c' < countCols grid

maybeExplodeBomb :: Grid -> Point -> Grid
maybeExplodeBomb grid (r, c) =
  if lookup (r, c) grid == Just (Bomb 0)
    then setMultiple grid (destroyed grid (r, c)) Empty
    else grid

maybeExplodeBombs :: Grid -> [Point] -> Grid
maybeExplodeBombs = foldl maybeExplodeBomb

explode :: Grid -> Grid
explode grid = maybeExplodeBombs grid ps
  where
    rows = countRows grid
    columns = countCols grid
    ps = [(r, c) | r <- [0 .. rows - 1], c <- [0 .. columns - 1]]

tick :: Grid -> Grid
tick = map (\(p, c) -> (p, maybeDecrement c))
  where
    maybeDecrement :: Cell Int -> Cell Int
    maybeDecrement (Bomb x) = Bomb (x - 1)
    maybeDecrement a = a

countRows :: Grid -> Int
countRows g = 1 + maximum (map (\(p, _) -> fst p) g)

countCols :: Grid -> Int
countCols g = 1 + maximum (map (\(p, _) -> snd p) g)

simulate :: (Grid, Int) -> Grid
simulate (g, sec)
  | sec < 1 = error "second should be >= 1"
  | sec == 1 = tick g
  | odd sec = explode $ tick $ simulate (g, sec - 1)
  | otherwise = plantBombs $ explode $ tick $ simulate (g, sec - 1)

getValue :: Grid -> Point -> Char
getValue grid (r, c)
  | val == Just Empty = '.'
  | isNothing val =
    error $
    show grid ++ " doesn't have point (" ++ show r ++ " " ++ show c ++ ")"
  | otherwise = 'O'
  where
    val = lookup (r, c) grid

printGrid :: Grid -> String
printGrid grid =
  runST $ do
    res <- newSTRef ""
    let r = countRows grid - 1
    let c = countCols grid - 1
    for_ [0 .. r] $ \ri -> do
      for_ [0 .. c] $ \ci ->
        modifySTRef' res (\v -> v ++ [getValue grid (ri, ci)])
      modifySTRef' res (\v -> v ++ ['\n'])
    readSTRef res

gridAndSecondsFromString :: String -> (Grid, Int)
gridAndSecondsFromString s =
  let ls = lines s
      [r, c, n] = map read $ words $ head ls :: [Int]
      grid = parseGrid (tail ls) r c
   in (grid, n)

main :: IO ()
main = interact $ printGrid . simulate . gridAndSecondsFromString
