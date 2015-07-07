using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace _2015_06_22__Challenge_220__Easy__Mangling_sentences {
    class Program {
        static void Main(string[] args) {
            string challengeInput;
            while ((challengeInput = Console.ReadLine()).Length != 0) {
                Console.WriteLine(challenge(challengeInput));
            }
        }

        static string challenge(string input) {
            string ret = input;
            List<char> workingWord = null;
            foreach (var word in input.Split(' ')) {
                Dictionary<int, bool> capital = new Dictionary<int, bool>();
                Dictionary<int, char> nonAlphaNumeric = new Dictionary<int, char>();
                for (int i = 0; i < word.Length; i++) {
                    if (char.IsUpper(word[i])) {
                        capital.Add(i, true);
                    }
                    if (!char.IsLetterOrDigit(word[i])) {
                        nonAlphaNumeric.Add(i, word[i]);
                    }
                }
                char[] toSort = Regex.Replace(word, @"[^a-zA-Z0-9]", "").ToLower().ToCharArray();
                Array.Sort(toSort);
                workingWord = toSort.ToList();
                foreach (KeyValuePair<int, char> entry in nonAlphaNumeric) {
                    workingWord.Insert(entry.Key, entry.Value);
                }
                foreach (KeyValuePair<int, bool> entry in capital) {
                    workingWord[entry.Key] = char.ToUpper(workingWord[entry.Key]);
                }
                ret = ret.Replace(word, String.Join("", workingWord.ToArray()));
            }

            return ret;
        }
    }
}