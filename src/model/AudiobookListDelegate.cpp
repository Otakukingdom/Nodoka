// Original by @Anthony Serdyukov posted on Stackoverflow
// http://stackoverflow.com/a/2039745/596065

#include "AudiobookListDelegate.h"

void AudiobookListDelegate::paint(QPainter *painter,
                                  const QStyleOptionViewItem &option,
                                  const QModelIndex &index) const {
    QStyleOptionViewItem optionV4 = option;
    initStyleOption(&optionV4, index);

    QStyle *style = optionV4.widget? optionV4.widget->style() : QApplication::style();

    QTextDocument doc;
    // if we don't have an empty string, then set the style sheet
    // for the QTextDocument
    if(this->styleSheet != "") {
        doc.setDefaultStyleSheet(this->styleSheet);
    }

    QTextOption textOption = doc.defaultTextOption();
    textOption.setWrapMode(QTextOption::WordWrap);
    doc.setDefaultTextOption(textOption);
    doc.setHtml(optionV4.text);
    doc.setTextWidth(optionV4.rect.width());

    /// Painting item without text
    optionV4.text = QString();
    style->drawControl(QStyle::CE_ItemViewItem, &optionV4, painter, optionV4.widget);

    QAbstractTextDocumentLayout::PaintContext ctx;

    // Highlighting text if item is selected
    if (optionV4.state & QStyle::State_Selected) {
        ctx.palette.setColor(QPalette::Text, optionV4.palette.color(QPalette::Active, QPalette::HighlightedText));
    } else {
        ctx.palette.setColor(QPalette::Text, optionV4.palette.color(QPalette::Active, QPalette::WindowText));
    }


    QRect textRect = style->subElementRect(QStyle::SE_ItemViewItemText, &optionV4);
    painter->save();
    painter->translate(textRect.topLeft());
    painter->setClipRect(textRect.translated(-textRect.topLeft()));
    doc.documentLayout()->draw(painter, ctx);
    painter->restore();
}

QSize AudiobookListDelegate::sizeHint(const QStyleOptionViewItem &option,
                                      const QModelIndex &index) const {
    QStyleOptionViewItem optionV4 = option;
    initStyleOption(&optionV4, index);

    QTextDocument doc;
    if(this->styleSheet != "") {
        doc.setDefaultStyleSheet(this->styleSheet);
    }
    doc.setHtml(optionV4.text);
    doc.setTextWidth(optionV4.rect.width());
    return QSize(doc.idealWidth(), doc.size().height());
;}

AudiobookListDelegate::AudiobookListDelegate(QString styleSheet) {
    this->styleSheet = styleSheet;
}

AudiobookListDelegate::AudiobookListDelegate() {
    this->styleSheet = "";
}
