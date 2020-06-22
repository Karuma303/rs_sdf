using System;

namespace Lib.SignedDistanceField {
	public class EightPointSignedSeqEuclideanDistTrans {
		private readonly int _width;
		private readonly int _height;
		private readonly bool[,] _data;

		private DistanceVector[,] _insideGrid;
		private DistanceVector[,] _outsideGrid;

		private readonly int _maxDistance;

		public EightPointSignedSeqEuclideanDistTrans(bool[,] data) {
			_width = data.GetLength(0);
			_height = data.GetLength(1);
			_data = data;

			_maxDistance = Math.Max(_width, _height) * 2; // the maximum (axis-)distance

			SetupGrids();
		}

		private void SetupGrids() {
			// we add two in each dimension to have an empty border
			_insideGrid = new DistanceVector[_width + 2, _height + 2];
			_outsideGrid = new DistanceVector[_width + 2, _height + 2];
		}

		public DistanceVector[,] calculate() {
			PopulateGrids();

			CalculateSdf(_insideGrid, true);
			CalculateSdf(_outsideGrid, false);

			return MergeGrids();
		}

		private void PopulateGrids() {
			PopulateGrid(_insideGrid, 0, _maxDistance, 0);
			PopulateGrid(_outsideGrid, _maxDistance, 0, _maxDistance);
		}

		private void PopulateGrid(DistanceVector[,] grid, int backgroundDistance, int foregroundDistance,
			int borderDistance) {
			// step #1 - add an empty border

			DistanceVector border = new DistanceVector(borderDistance, borderDistance, false);

			for (int y = 0; y < _height + 2; y++) {
				grid[0, y] = border; // left
				grid[_width + 1, y] = border; // right
			}

			for (int x = 0; x < _width; x++) {
				grid[x + 1, 0] = border;
				grid[x + 1, _height + 1] = border;
			}

			DistanceVector foreground = new DistanceVector(foregroundDistance, foregroundDistance, true);
			DistanceVector background = new DistanceVector(backgroundDistance, backgroundDistance, false);

			for (int y = 0; y < _height; y++)
			for (int x = 0; x < _width; x++)
				grid[x + 1, y + 1] = _data[x, y] ? foreground : background;
		}

		private void CalculateSdf(DistanceVector[,] grid, bool discardType) {
			// forward pass
			for (int y = 0; y < _height; y++) {

			    // ***
			    // *O.
			    // ...
				for (int x = 0; x < _width; x++) {
					Compare(grid, x, y, -1, 0, discardType);
					Compare(grid, x, y, 0, -1, discardType);
					Compare(grid, x, y, -1, -1, discardType);
					Compare(grid, x, y, 1, -1, discardType);
				}

                // ...
                // .O*
                // ...
				for (int x = _width - 1; x >= 0; x--) {
					Compare(grid, x, y, 1, 0, discardType);
				}
			}

			// backward pass
			for (int y = _height - 1; y >= 0; y--) {

			    // ...
			    // .O*
			    // ***
				for (int x = _width - 1; x >= 0; x--) {
					Compare(grid, x, y, 1, 0, discardType);
					Compare(grid, x, y, 0, 1, discardType);
					Compare(grid, x, y, -1, 1, discardType);
					Compare(grid, x, y, 1, 1, discardType);
				}

			    // ...
			    // *O.
			    // ...
				for (int x = 0; x < _width; x++) {
					Compare(grid, x, y, -1, 0, discardType);
				}
			}
		}

		//compares a pixel for the sweep, and updates it with a new distance if necessary
		void Compare(DistanceVector[,] grid, int x, int y, int xoffset, int yoffset, bool discardType) {
			x++;
			y++;

			//calculate the location of the other pixel, and bail if in valid
			int otherX = x + xoffset;
			int otherY = y + yoffset;

			//read the distance values stored in both this and the other pixel
			DistanceVector current = grid[x, y];
			DistanceVector other = grid[otherX, otherY];

			// if (other.Dx == _maxDistance && other.Dy == _maxDistance) return;
			if (other.IsInside == discardType) return;

			other.Dx -= xoffset;
			other.Dy -= yoffset;

			if (other.DistSq() < current.DistSq())
				grid[x, y] = other;
		}

		private DistanceVector[,] MergeGrids() {
			DistanceVector[,] merged = new DistanceVector[_width, _height];
			for (int y = 0; y < _height; y++) {
				for (int x = 0; x < _width; x++) {
					if (_data[x, y])
						merged[x, y] = new DistanceVector(_insideGrid[x + 1, y + 1], true);
					else
						merged[x, y] = new DistanceVector(_outsideGrid[x + 1, y + 1], false);
				}
			}

			return merged;
		}

		/// <summary>
		/// Simple data structure that defines a (signed) distance vector in a vector field.
		/// The sign of the vector is determined by the IsInside property.
		/// </summary>
		public struct DistanceVector {
			public int Dx;
			public int Dy;
			public bool IsInside;

			public DistanceVector(int dx, int dy, bool isInside) {
				Dx = dx;
				Dy = dy;
				IsInside = isInside;
			}

			public DistanceVector(DistanceVector p, bool isInside) {
				Dx = p.Dx;
				Dy = p.Dy;
				IsInside = isInside;
			}

			/// <summary>
			/// The squared distance of the vector.
			/// </summary>
			/// <returns>The sqared distance</returns>
			public int DistSq() {
				return Dx * Dx + Dy * Dy;
			}
		}
	}
}