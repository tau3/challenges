{-# LANGUAGE LambdaCase #-}

import Control.Monad
import Data.List
import qualified Data.Map as Map
import qualified Data.Set as Set

dfs :: Int -> Map.Map Int [Int] -> Set.Set Int -> Set.Set Int
dfs vertice adjacencyMatrix visited =
  foldl'
    (\currentVisited adjacentVertice ->
       dfs adjacentVertice adjacencyMatrix currentVisited)
    visited'
    toVisit
  where
    visited' = Set.insert vertice visited
    adjacents = Map.findWithDefault [] vertice adjacencyMatrix
    toVisit = filter (`Set.notMember` visited') adjacents

splitTrees :: [Int] -> Map.Map Int [Int] -> [Int]
splitTrees vertices adjacencyMatrix = go [] vertices Set.empty
  where
    go res (v:vs) visited
      | v `Set.notMember` visited =
        let visited' = dfs v adjacencyMatrix visited
         in go
              ((length visited' - length visited) : res)
              vs
              (dfs v adjacencyMatrix visited)
      | otherwise = go res vs visited
    go res [] _ = res

perms :: [Int] -> Int
perms xs =
  sum $ map (uncurry (*)) [(snd i, snd j) | i <- xs', j <- xs', fst i < fst j]
  where
    xs' = zip ([0 ..] :: [Int]) xs

makeAdjacencyMatrix :: [[Int]] -> Map.Map Int [Int]
makeAdjacencyMatrix = foldl' (\m [u, v] -> insertEdge m u v) Map.empty
  where
    insertEdge m u v = insert' (insert' m v u) u v
    insert' :: Map.Map Int [Int] -> Int -> Int -> Map.Map Int [Int]
    insert' m' k' v' =
      Map.alter
        (\case
           Just vs -> Just (v' : vs)
           Nothing -> Just [v'])
        k'
        m'

main :: IO ()
main = do
  [n, p] <- map read . words <$> getLine :: IO [Int]
  edges <- (map (map read . words) <$> replicateM p getLine) :: IO [[Int]]
  let trees = splitTrees [0 .. n - 1] (makeAdjacencyMatrix edges)
  print $ perms trees
