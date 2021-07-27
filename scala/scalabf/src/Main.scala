import scala.io.StdIn.{readLine, readChar};

case class VmState(ip: Int, labels: Array[Int], memory: Array[Char], ptr: Int);

def execute(code: String) = {
  runTurn(code, VmState(0, Array(), Array[Char](1).updated(0, 0), 0));
}
private def runTurn(code: String, state: VmState): Unit = {
	if (state.ip >= code.length) return;
	val pointedVal = state.memory(state.ptr);
	code.charAt(state.ip) match {
    case '>' => runTurn(code, VmState(state.ip + 1, state.labels, if (state.memory.length <= state.ptr + 1) state.memory :+ 0 else state.memory, state.ptr + 1));
    case '<' => runTurn(code, VmState(state.ip + 1, state.labels, state.memory, if (state.ptr == 0) state.memory.length - 1 else state.ptr - 1));
    case '+' => runTurn(code, VmState(state.ip + 1, state.labels, state.memory.updated(state.ptr, if (pointedVal == 255) 0 else (pointedVal + 1).toChar), state.ptr));
    case '-' => runTurn(code, VmState(state.ip + 1, state.labels, state.memory.updated(state.ptr, if (pointedVal == 0) 255 else (pointedVal - 1).toChar), state.ptr));
    case '[' => runTurn(code, VmState(state.ip + 1, state.labels :+ state.ip, state.memory, state.ptr));
    case ']' => runTurn(code, VmState(if (pointedVal > 0) state.labels.last else state.ip + 1, if (pointedVal == 0) state.labels.init else state.labels, state.memory, state.ptr));
    case '.' => {
      print(pointedVal);
      runTurn(code, VmState(state.ip + 1, state.labels, state.memory, state.ptr));
    }
    case ',' => runTurn(code, VmState(state.ip + 1, state.labels, state.memory.updated(state.ptr, readChar), state.ptr));
    case _ => runTurn(code, VmState(state.ip + 1, state.labels, state.memory, state.ptr)),
  }
}

object Main {
	def main(_args: Array[String]): Unit = {
		execute("""++[->++++<]>+[->++++++++++<]>---.
			<++[->+++++++<]>.
			<++[->+++<]>+.
			<++[->----<]>-.
			<++++[->+++<]>.
			--.
			<++[->----<]>.
			<+++++++[->----------<]>+.
			<++++++++++[->++++++++<]>++++.
			-----.
			<++++++++++[->--------<]>+.
			<++++++++++[->+++++<]>+.
			<++++[->++++<]>.
			--.
			<+++++[->++<]>+.
			<+++++[->--<]>-.
			<++++++++[->----<]>+.
			++++.
			>+++++[->++<]>.""");

		while (true) {
			execute("+++[->++++<]>[->+++++<]>++.<++++++[->-----<]>.")
			execute(readLine);
		}
	}
}
