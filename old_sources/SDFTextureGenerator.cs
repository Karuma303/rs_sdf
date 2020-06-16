using System;
using UnityEngine;

namespace Lib.SignedDistanceField {
	public class SDFTextureGenerator {

		public static Texture2D fromTexture(Texture2D source) {

			bool[,] data = new bool[source.width, source.height];

			for (int y = 0; y < source.height; y++)
				for (int x = 0; x < source.width; x++)
					data[x, y] = source.GetPixel(x, y).grayscale > 0.5f;

			EightPointSignedSeqEuclideanDistTrans trans = new EightPointSignedSeqEuclideanDistTrans(data);
			EightPointSignedSeqEuclideanDistTrans.DistanceVector[,] result = trans.calculate();

			Texture2D resText = new Texture2D(source.width, source.height, TextureFormat.RGB24, false);


			for (int y = 0; y < source.height; y++) {
				for (int x = 0; x < source.width; x++) {
					EightPointSignedSeqEuclideanDistTrans.DistanceVector p = result[x, y];
					double val = Math.Sqrt(p.DistSq());
					double clamped = Mathf.Clamp((float)val, 0, 255);
					byte grey = (byte) clamped;
					Color32 col;
					// green is inner / blue is outer
					if(p.IsInside)
						col = new Color32(0, grey, 0, Byte.MinValue);
					else
						col = new Color32(0, 0, grey, Byte.MinValue);
					resText.SetPixel(x, y, col);
				}
			}

			resText.Apply();
			return resText;
		}
	}
}