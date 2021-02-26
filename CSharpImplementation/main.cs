using System;
using System.Globalization;
using System.Collections.Generic;

namespace Program {
    class RPNCalculator {
        static void Main(String[] args) {
            String line = System.Console.ReadLine();
            String[] instructions = line.Split(' ');
            Stack<double> stack = new Stack<double>();

            Dictionary<String, Func<double, double, double>> op_codes = new Dictionary<String, Func<double, double, double>>(); 

            op_codes.Add("+", new Func<double, double, double>((a, b) => a + b));
            op_codes.Add("-", new Func<double, double, double>((a, b) => a - b));
            op_codes.Add("/", new Func<double, double, double>((a, b) => a / b));
            op_codes.Add("*", new Func<double, double, double>((a, b) => a * b));

            double number;
            Func<double, double, double> action;

            foreach(String instruction in instructions) {
                if(Double.TryParse(instruction, out number)) {
                    stack.Push(number);
                } else {
                    if(op_codes.TryGetValue(instruction, out action)) {
                        double b = stack.Pop(), a = stack.Pop();
                        stack.Push(action(a, b));
                    }
                }
            }

            System.Console.WriteLine($"Last value on the stack: {stack.Peek()}");
            System.Console.WriteLine($"Stack dump: {string.Join(" ", stack)}");
        }
    }
}