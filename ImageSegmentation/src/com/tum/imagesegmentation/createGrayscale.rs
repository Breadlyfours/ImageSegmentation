/*
 * Copyright (C) 2014 Sebastian Soyer
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#pragma version(1)
#pragma rs java_package_name(com.tum.imagesegmentation)

#include "rs_debug.rsh" 

rs_allocation gIn;
rs_allocation gOut;
rs_script gScript;

void root(const uchar4 *v_in, uchar4 *v_out, const void *usrData, uint32_t x, uint32_t y) {

    float4 pixel = rsUnpackColor8888(*v_in);
    float newValue = 0.2126 * pixel.r + 0.7152 * pixel.g + 0.0722 * pixel.b;
    *v_out = rsPackColorTo8888(newValue, newValue, newValue, pixel.a);
    
}


void filter() {
    rsForEach(gScript, gIn, gOut, 0, 0);
}