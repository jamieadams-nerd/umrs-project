# Policy text extractor for TBS/GoC HTML policy documents
# Usage: curl ... | python3 extract_pol.py
# Or: python3 extract_pol.py <file.html>
import sys
import re
from html.parser import HTMLParser


class PolicyExtractor(HTMLParser):
    def __init__(self):
        super().__init__()
        self.skip_tags = {'script', 'style'}
        self.skip_depth = 0
        self.text_parts = []
        self.in_policy = False

    def handle_starttag(self, tag, attrs):
        attrs_dict = dict(attrs)
        if tag in self.skip_tags:
            self.skip_depth += 1
            return
        ids = attrs_dict.get('id', '')
        classes = attrs_dict.get('class', '')
        # Start collecting at main content div
        if ids in ('ps-doc', 'wb-cont') or 'pol-cha' in classes or 'mrgn-tp-lg' in classes:
            self.in_policy = True
        if not self.in_policy:
            return
        if tag in ('h1', 'h2'):
            self.text_parts.append('\n\n# ')
        elif tag in ('h3', 'h4'):
            self.text_parts.append('\n\n## ')
        elif tag in ('h5', 'h6'):
            self.text_parts.append('\n\n### ')
        elif tag == 'li':
            self.text_parts.append('\n- ')
        elif tag in ('p', 'section', 'details', 'summary', 'article'):
            self.text_parts.append('\n')
        elif tag == 'br':
            self.text_parts.append('\n')
        elif tag == 'td':
            self.text_parts.append(' | ')
        elif tag == 'tr':
            self.text_parts.append('\n')

    def handle_endtag(self, tag):
        if tag in self.skip_tags and self.skip_depth > 0:
            self.skip_depth -= 1

    def handle_data(self, data):
        if self.skip_depth > 0:
            return
        if not self.in_policy:
            return
        text = data.strip()
        if text:
            self.text_parts.append(text + ' ')


def main():
    if len(sys.argv) > 1:
        with open(sys.argv[1], 'r', encoding='utf-8', errors='replace') as f:
            html = f.read()
    else:
        html = sys.stdin.read()

    p = PolicyExtractor()
    p.feed(html)
    result = ''.join(p.text_parts)
    # Clean up excessive whitespace
    result = re.sub(r' {2,}', ' ', result)
    result = re.sub(r'\n{4,}', '\n\n\n', result)
    print(result)


if __name__ == '__main__':
    main()
