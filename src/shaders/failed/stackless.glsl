// float PrepareStackPlaneMarch(vec3 pos, vec3 d) {
  // return StackPlaneMarch((pos / 32) + vec3(45, 15, 45) / 2.0, d, ivec3(0, 2, 0));
// }


MarchResult StackPlaneMarch(vec3 p0, vec3 d, ivec3 offset) {
  // constants
  vec3 invDir = vec3(1.0 / d.x, 1.0 / d.y, 1.0 / d.z);
  vec3 corner = step(0, d);

  bvec3 mask;

  vec3 boxMax = vec3(32);

  int mip_level = 3; // currently supports 0, 1, 2, 3
  int depth = 0;

  ivec3 offsets[2] = {
    ivec3(offset),
    ivec3(0),
    ivec3(0)
  };
  vec3 p[2] = {
    vec3(p0),
    vec3(0),
    vec3(0)
  };

  for (int i = 0; i < 300; i++) {
    if (depth == 0) {
      boxMax = vec3(45, 15, 45);
    } else {
      boxMax = vec3(32);
    }
    // evaluate conditions
    // these do NOT change throughout the loop
    bool pInBox = insideBox3D(p[depth], vec3(0), boxMax);
    bool voxelFound = pInBox;
    if (pInBox) {
      voxelFound = checkVoxel(ivec3(p[depth]) + offsets[depth] * 32, mip_level);
    }
    
    // return no hit
    if (!pInBox && depth == 0) {
      return MarchResult (p[0], false);
    }

    // return hit
    if (voxelFound && mip_level == 0 && depth == 2) {
      return MarchResult (p[0] + (p[1] / 32.0 - fract(p[0])), true);
    }

    // pop stack
    // expect advancement if this executes
    if (!pInBox && depth != 0) {
      depth -= 1;
      mip_level = 0;
    }

    // push stack
    if (voxelFound && mip_level == 0) {
      int chunkIndex = int(imageLoad(megaIndexMap, ivec3(p[depth]) + offsets[depth] * 32));
      ivec3 pushOffset = ivec3(chunkIndex % 32, chunkIndex / (1024), (chunkIndex / 32) % 32);
      vec3 chunkPos = fract(p[depth]) * 32.0;
      vec3 towardsChunkMiddle = normalize(vec3(16.0) - chunkPos);
      vec3 adjustedChunkPos = chunkPos + towardsChunkMiddle * 0.0001;

      p[depth + 1] = adjustedChunkPos;
      offsets[depth + 1] = pushOffset;

      depth += 1;
      mip_level = 3 + 1; // additional plus one because of minus at the end
    }

    // advance position
    // mip_level and depth may change from iteration start!
    // can occur after a pop
    if (!voxelFound){ 
      float width = float(pow(2, mip_level));
      vec3 deltas = (corner * width - mod(p[depth], width)) * invDir;
      mask = lessThanEqual(deltas.xyz, min(deltas.yzx, deltas.zxy));
      float minDelta = dot(vec3(mask) * deltas, vec3(1.0));
      p[depth] += (minDelta + 0.001) * d;
    }

    mip_level += int(!voxelFound) * 2 - 1;
    mip_level = clamp(mip_level, 0, 3);
  }

  // normal = -sign(d) * vec3(mask);

  return MarchResult (vec3(0.0), false);
}

