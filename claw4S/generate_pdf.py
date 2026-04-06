#!/usr/bin/env python3
"""Generate paper.pdf from paper.md using fpdf."""

import re
import os
from fpdf import FPDF

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
PAPER_MD = os.path.join(SCRIPT_DIR, "paper.md")
PAPER_PDF = os.path.join(SCRIPT_DIR, "paper.pdf")


class PaperPDF(FPDF):
    def header(self):
        if self.page_no() > 1:
            self.set_font("Helvetica", "I", 8)
            self.cell(0, 10, "Clawling: Architecture and Early Population Dynamics", 0, 0, "C")
            self.ln(5)

    def footer(self):
        self.set_y(-15)
        self.set_font("Helvetica", "I", 8)
        self.cell(0, 10, "Page %d/{nb}" % self.page_no(), 0, 0, "C")


def clean_text(text):
    """Remove markdown formatting and replace non-latin1 chars."""
    text = re.sub(r"\*\*(.*?)\*\*", r"\1", text)
    text = re.sub(r"`(.*?)`", r"\1", text)
    # Replace common Unicode chars with latin1 equivalents
    text = text.replace("\u2014", "--")  # em dash
    text = text.replace("\u2013", "-")   # en dash
    text = text.replace("\u2018", "'")   # left single quote
    text = text.replace("\u2019", "'")   # right single quote
    text = text.replace("\u201c", '"')   # left double quote
    text = text.replace("\u201d", '"')   # right double quote
    text = text.replace("\u2026", "...")  # ellipsis
    # Encode to latin1, replacing anything else
    text = text.encode("latin-1", errors="replace").decode("latin-1")
    return text


def render_paper():
    with open(PAPER_MD, "r", encoding="utf-8") as f:
        content = f.read()

    pdf = PaperPDF()
    pdf.alias_nb_pages()
    pdf.set_auto_page_break(auto=True, margin=20)
    pdf.add_page()

    lines = content.split("\n")
    in_table = False
    in_code = False

    for line in lines:
        # Code blocks
        if line.strip().startswith("```"):
            in_code = not in_code
            if in_code:
                pdf.set_font("Courier", "", 9)
            else:
                pdf.set_font("Helvetica", "", 11)
            continue

        if in_code:
            pdf.set_font("Courier", "", 9)
            pdf.set_fill_color(240, 240, 240)
            text = clean_text(line)
            pdf.cell(0, 5, text, 0, 1, fill=True)
            continue

        # Table detection
        if "|" in line and line.strip().startswith("|"):
            if re.match(r"^\|[\s\-|]+\|$", line.strip()):
                continue  # Skip separator row
            cells = [clean_text(c.strip()) for c in line.strip().strip("|").split("|")]
            if not in_table:
                in_table = True
                pdf.set_font("Helvetica", "B", 9)
            else:
                pdf.set_font("Helvetica", "", 9)
            col_width = (pdf.w - 20) / max(len(cells), 1)
            for cell in cells:
                pdf.cell(col_width, 6, cell, 1, 0)
            pdf.ln()
            continue
        else:
            in_table = False

        # Title (h1)
        if line.startswith("# ") and not line.startswith("## "):
            pdf.set_font("Helvetica", "B", 16)
            pdf.multi_cell(0, 8, clean_text(line[2:].strip()))
            pdf.ln(3)
            continue

        # Section headers (h2)
        if line.startswith("## "):
            pdf.ln(5)
            pdf.set_font("Helvetica", "B", 14)
            pdf.multi_cell(0, 7, clean_text(line[3:].strip()))
            pdf.ln(2)
            continue

        # Subsection headers (h3)
        if line.startswith("### "):
            pdf.ln(3)
            pdf.set_font("Helvetica", "B", 12)
            pdf.multi_cell(0, 6, clean_text(line[4:].strip()))
            pdf.ln(1)
            continue

        # Bold author line
        if line.startswith("**Author:"):
            pdf.set_font("Helvetica", "B", 11)
            text = clean_text(line)
            pdf.cell(0, 6, text, 0, 1)
            pdf.ln(2)
            continue

        # Numbered list items
        if re.match(r"^\d+\.\s", line.strip()):
            pdf.set_font("Helvetica", "", 11)
            pdf.multi_cell(0, 6, clean_text(line.strip()))
            continue

        # Bullet list items
        if line.strip().startswith("- "):
            pdf.set_font("Helvetica", "", 11)
            text = clean_text(line.strip()[2:])
            pdf.cell(5, 6, "", 0, 0)
            pdf.multi_cell(0, 6, "  - " + text)
            continue

        # Empty line
        if line.strip() == "":
            pdf.ln(3)
            continue

        # Regular paragraph
        pdf.set_font("Helvetica", "", 11)
        pdf.multi_cell(0, 6, clean_text(line))

    pdf.output(PAPER_PDF, "F")
    print("Generated: %s" % PAPER_PDF)


if __name__ == "__main__":
    render_paper()
