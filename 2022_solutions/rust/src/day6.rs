use crate::signals::{start_of_message_marker, start_of_packet_marker};

fn puzzle1(input_string: &str) -> usize {
    return start_of_packet_marker(input_string);
}

fn puzzle2(input_string: &str) -> usize {
    return start_of_message_marker(input_string);
}

#[cfg(test)]
mod tests {
    use crate::day6::puzzle1;
    use crate::day6::puzzle2;

    const TEST_DATA_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const TEST_DATA_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const TEST_DATA_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const TEST_DATA_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const TEST_DATA_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    const PUZZLE_DATA: &str = "htsslsmstsrrhlrrllqppfnpnzzqtqtjqttslttmvmsmbbnpbbznzjjmrmsrrnjjzczcfcqchcnnhrnrzrnrzzddtrrpjrprbpbwbswsqswqswsqsqffgngtgwtwbtbbhhslsrsmrmffgcgtcgcppnbpnpbnpptcptpltthjhwwlttrrlldzldzzzlssccrfrqfrfdfldfldfdzfdzzcnznwznzqqzpqzqdzzbbslbljlqjjvzjzgzqgqqqvsvffzjfzfmzzrjrhjhrhcrhhtgtmggchggcsggvtttwmmsspmpffzpfpjjnwwnpwwdttcfcmmlblwlvvqrqddcdwcwnnfqnqdnnncggflfdftddtftqthqhrrmsrmsrsffbccjnnjjgddppjmmldllhttqvqzvzrvrsslnlplrprtprtttnvnsvsvzzdndrnnnlznzcnnzwwzjjsnjnwjwqqczqcqwqppqnqqllgblbhhbbzbjzzwjzzrqzqggdppgcgwcclglhhwwgcccgcvvpbbrzzwszsqqjqrrczzvmvpprllsgstsnnnrvvsszgzddzttbccmjccwtccrbccnssdqsdsqqfgqfqbfqfjqqzqqmhmwhwffsjjcjffqmmwjwhjjqttpvvgddzjdddwndwdjdldgdsdmdzmdzmzqqszzcwwwjbbvvlplnpndnddjcjcttrgrggjvjsjvjssnsmshmhrmhhfvvdqqppmbbjnbnzzzwlzzfmmlttvptvpphlhwhzzwffcsfsppdssvsvgsgcscmmdwwpggtcgttmlmglgmmcsccmmgffnvvvvsscstctptzzcmcsclcffnwwndnwddmgmfmvffntfnndgdrrbmrrvprrjnjcnjjsgjgmmjjjqwqhhlfljfljlclttrgttmdmhhcncwncnmccnmnqqmhmwmdwdmdfdttnggcscfsftttcjchhqghhtdthtwtgwgttlffphfhtftpfttlgtltblljmljjsmsszgsghhnnmvnmnvmvmhvvtmmdqdppclplqljllzppwgpgvpvjpvvlwldlcdcmvfrbcqfdrgqjdmpvczgmtrbhtfgbctjghwfmpqtprlnvzvbmhnmgsnbcqpdqhtstncmtrvwmjzzmsbprfvmszgvdwjwchzcvmzncfblffwvwwrdqpfnvrtlftnvhcqpwfgzdnmpqthblmqzhbdbdhmqvscfrpbjbgjgdndhlnnpcrrmrlghcwvmmmhbvdsrztbpdvhwhsphmgblzfpflhnbrvjsgsbvgcfwdlbmbsmbqfwczhhbbwntsczhljgvdwpsfmjppmdwcfchhvtrwlfgqrzbtzndqwbwqflpthhgtzrfmspfnlrrvrjttwnzzfsbzlvfwlntvcqhfntllsrdgqjdbrbvnmjgfsqshmcgtsmlrzjmmqbpfdfhlghqjnjrcnzvvmbwhwfwpbbsmpdmvvznstqpjwmthncjsvfmbhwtbfqgfmvhhwrwzrzccbmmrngfbcqgsdjlfwrlzqwrbhvhmjzqqtgqvjwgllhhgphjthwnscndjfhrdzjctthwfszztvtzcnvsnfdwnvrrbjzplgnghfsvnflhctjntrpzfgjnfzlmwzrvqcfdqqscrffjshhgmvlgdtqpbllfdspdhcccqbmpqcgctljwzdszbfzsltfmdrcvmjqmpvzzhnczlfdbrmsvfffmfzjvnthghqgtnvjtflzrrnrhqshtzslbmbghvtqcmbdgzcmmrpbtqlzbzjhhgfcbrbcztlhdcfqccqsgdqdfrvlsprbhwhtmjfbzlzfpcslcddzdhbhstvqjgdfwdwzfhrzpthnzgmzrrbrszvcwhbdczzcnqsqrwdpmgdmcdmccmfdwdfmzpbbsdtjlmlvcnvhhgcltwmpzqmjlpbsbfrrntrbmrrmgqrdsbfcgnqvnmwwgnmvvpjzpwbvwfzzfcfrbgdshrvbpdlbpzdsvjnrsmfttqgbngjbqcnhhhmrnlgwjnlfcsdwccqqlzcdrcqnjlmsfqldgdmwlctstcvqbvvwvhvrnhvtgpcnsstnhvlttdnstlndnpdglqlbgggqlwqpfztgzvhqqwbctvgtrmrbpmvwlztsfbrmhhpfdtnsjcjmngsqtzvhqjnwmgcrjbhvghpnrjlrhjfrfrmbzvprpzlcshbqtlnlmqmfhsmbznjmpzwljccngbsvqqmghqsqwwmtqhgddnpcmmlfvmplgptzmfsmpntprnwlpjmsdsntpmpnhwlqgfwslrnjlmvhvqsnlcssvrqvtvfhhghbhnvmpngtdmrcmftjmwgnptbmjcwvrqqbczmtdprlltcjcggvffvjwdthbhvjvhbsmfnstqvczmvsgsqfjmddszzrwnbrbvlhgltjddczwpzpsgfgbtmgmssbszhjfbprpbgsqpvgwfnmpcdwfzwbfbtfwhjbjgrctcwqgfhpvjpjssmcpppwpbnlfsrmbzqdpmpmqjtzqmqswpfhfvfltwnfbltvdmfgmpbhzzdrbtnmmjfqgtrgmrbsgplfrmlnjrggtslngnphcvcwvqsdftlhddhpsdwlhzcrfwgwbndplwjmtrltmswwdqjpgmmcdchllzdgpfpfrvqrmvppvzcrvbswzdclnqnqvmfvjtpzpvzlmbngtrrjwpgqjgrrqbqwcvjlgqzddtcfmmrqtnjppqvztfpdrdmjpsqqvzrhwjgvdwwdtmrzrvhwgsqvjjrtsdtwfwhrbcsqdblhgwzghrqrqtfbzszrmjqbhrtsrcfjlzbcjmdnpthrtbhtzmhgpfcqndwpmtlvzschzzcqdnzdrfhczsrscvttlpslrzgvwprwppmpfjqwhtnhzcdjjwmsnvqzmtlzsdpbdtdtsmbmjszsglrldcsgtnmbgsjfmnrftnmvnjtswrmdthvstmdlsnsmrbqhdlpnmnhjhcccgnzrrsljvwswmhjqrrfbwwhgrcttsdcjsdtlgmgblfnvwmgztbbzlbrdnspfnvwvhzlztdhlzhwwppgwwvrhfbjvrmjpjflqzbdnlvptmrsggqmzgmzlsdzbfqnqzzdnzfgmhncvwmgrcfrmlnwzcwdsvtwbvcqpmgcczbnnfdrlsgnfgnpbdstdmwhlprnzvhjpznsgwhfhzfwmjsvcbwccpsnfgbzrltcbwczcmzwljcswgwjnhwtjrjvgrggsrqszscrjcghnwdzpzhttrjcgwvmtcwmrhvwptlgfjpdjpnhphmzdgfdvsncswbdvjdtsgdtlsgjljlczgrwbswtbwrcfpmhgfjfnhpsmfrrtcjmpvvscmgftpprdmbjwcvhzbrmpbvhgzwftwtvvhrjmljgjhbpnbnfntmjhvjrzwlqbtqlrfbblgmfsbgzhhscgwgzrwvflnfctngtwbvdbbgspntclnbwgpppjgbrlqvtfznpdttrsplvjfsgbjwjprbvdfvjtffbhsjwgbsfschfnmlqdfzmdwzvfctjjvzncdvrdttlpvpgvsssflpfnrzgfznzlldrbgnnztngtlbbrmrmlfnnspsvvsfzfbmsblmzdwqcrftvnbvdlhgdqjtglclpzrchtlffrfwslqjvbfpvnhmdgqrcjtbhmjqwqzpfndqbwldbzmwqsptdccczmhwmdqqzcqvmbbnmqndtspmbtggcghdhsfgrjgvwjdbwmltbhdvdtgqnpqhwmhzpzbqjtnlftqrjbsvtwtgpmvpnfwdtsjgdtfnlntpgrmwphphrrzhbdrbzhtqddwvptdllntjzldzrndhfjwdfnmtjmjtfmndbvlwgmlcmwwlpfdjwbfznbllbmqrlmvljngvpmdlqmvdvwvpqwlbqssqcbnrmhdvrzwljstghmhntwsbqmnlpgthbwmrznbbggthtjhnndnqbzmcrtftcbnpctqjzghdfcvmmvpqwtnntstlspfsgwfdbrlsbwgbhhbfcvwrjclsgmmbqmmjwtdjqppjvcbnbvfwczlqzbtnlhzhssglgnlm";

    #[test]
    fn test_puzzle1() {
        let return_value = puzzle1(TEST_DATA_1);
        assert_eq!(return_value, 7);
    }

    #[test]
    fn test_puzzle1_2() {
        let return_value = puzzle1(TEST_DATA_2);
        assert_eq!(return_value, 5);
    }

    #[test]
    fn test_puzzle1_3() {
        let return_value = puzzle1(TEST_DATA_3);
        assert_eq!(return_value, 6);
    }

    #[test]
    fn test_puzzle1_4() {
        let return_value = puzzle1(TEST_DATA_4);
        assert_eq!(return_value, 10);
    }

    #[test]
    fn test_puzzle1_5() {
        let return_value = puzzle1(TEST_DATA_5);
        assert_eq!(return_value, 11);
    }

    #[test]
    fn test_puzzle1_realdata() {
        let return_value = puzzle1(PUZZLE_DATA);
        assert_eq!(return_value, 1080);
    }

    #[test]
    fn test_puzzle2() {
        let return_value = puzzle2(TEST_DATA_1);
        assert_eq!(return_value, 19);
    }

    #[test]
    fn test_puzzle2_2() {
        let return_value = puzzle2(TEST_DATA_2);
        assert_eq!(return_value, 23);
    }

    #[test]
    fn test_puzzle2_3() {
        let return_value = puzzle2(TEST_DATA_3);
        assert_eq!(return_value, 23);
    }

    #[test]
    fn test_puzzle2_4() {
        let return_value = puzzle2(TEST_DATA_4);
        assert_eq!(return_value, 29);
    }

    #[test]
    fn test_puzzle2_5() {
        let return_value = puzzle2(TEST_DATA_5);
        assert_eq!(return_value, 26);
    }

    #[test]
    fn test_puzzle2_realdata() {
        let return_value = puzzle2(PUZZLE_DATA);
        assert_eq!(return_value, 3645);
    }
}