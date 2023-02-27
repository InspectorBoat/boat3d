package boat.util

import boat.world.Chunk.Face
import org.lwjgl.opengl.GL11C
import org.lwjgl.opengl.GL20C
import java.nio.IntBuffer

fun createShader(type: Int, source: String?): Int {
    val shader = GL20C.glCreateShader(type)
    GL20C.glShaderSource(shader, source)
    GL20C.glCompileShader(shader)
    val status = IntArray(1)
    GL20C.glGetShaderiv(shader, GL20C.GL_COMPILE_STATUS, status)
    if (status[0] == GL11C.GL_TRUE) return shader
    throw RuntimeException("Failed to compile " + (if (type == GL20C.GL_FRAGMENT_SHADER) "fragment" else "vertex") + " shader")
}

fun createProgram(vertShader: Int, fragShader: Int): Int {
    val program = GL20C.glCreateProgram()
    GL20C.glAttachShader(program, vertShader)
    GL20C.glAttachShader(program, fragShader)
    GL20C.glLinkProgram(program)
    val status = IntArray(1)
    GL20C.glGetProgramiv(program, GL20C.GL_LINK_STATUS, status)
    if (status[0] == GL11C.GL_TRUE) return program
    throw RuntimeException("Failed to link program")
}

fun sortBuffer(buffer: IntBuffer, array: IntArray, counts: IntArray, startIndex: Int, endIndex: Int) {
    var i = 0
    var sum = startIndex
    while (i < counts.size) {
        val prev = counts[i]
        counts[i] = sum
        sum += prev
        i++
    }
    var pos = startIndex
    while (pos < endIndex) {
        val n = array[pos]
        val v = buffer[pos]
        val dest = counts[n and 0xf]
        if (dest <= pos) {
            if (dest < pos) {
                buffer.put(pos, buffer[dest])
                buffer.put(dest, v)
                array[pos] = array[dest]
                array[dest] = n
            } else ++pos
            counts[n and 0xf] = dest + 1
        } else ++pos
    }
}

fun dir(face: Face?, x: Byte, y: Byte, z: Byte): Int {
    when (face) {
        Face.SOUTH -> {
            return if (z <= 0) (x.toInt() shl 8 or (y.toInt() shl 4) or (15 shl 0)) + 8192 else x.toInt() shl 8 or (y.toInt() shl 4) or (z - 1 shl 0)
        }

        Face.NORTH -> {
            return if (z >= 15) (x.toInt() shl 8 or (y.toInt() shl 4) or (0 shl 0)) + 8192 else x.toInt() shl 8 or (y.toInt() shl 4) or (z + 1 shl 0)
        }

        Face.WEST -> {
            return if (x <= 0) (15 shl 8 or (y.toInt() shl 4) or (z.toInt() shl 0)) + 8192 else x - 1 shl 8 or (y.toInt() shl 4) or (z.toInt() shl 0)
        }

        Face.EAST -> {
            return if (x >= 15) (0 shl 8 or (y.toInt() shl 4) or (z.toInt() shl 0)) + 8192 else x + 1 shl 8 or (y.toInt() shl 4) or (z.toInt() shl 0)
        }

        Face.DOWN -> {
            return if (y <= 0) (x.toInt() shl 8 or (15 shl 4) or (z.toInt() shl 0)) + 8192 else x.toInt() shl 8 or (y - 1 shl 4) or (z.toInt() shl 0)
        }

        Face.UP -> {
            return if (y >= 15) (x.toInt() shl 8 or (0 shl 4) or (z.toInt() shl 0)) + 8192 else x.toInt() shl 8 or (y + 1 shl 4) or (z.toInt() shl 0)
        }

        else -> { throw RuntimeException() }
    }
}