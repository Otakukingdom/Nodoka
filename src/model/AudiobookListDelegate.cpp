// Original by @Anthony Serdyukov posted on Stackoverflow
// http://stackoverflow.com/a/2039745/596065

#include "AudiobookListDelegate.h"
#include <QDebug>

// We need to set this manually because we can't easily set the QColor from stylesheets
// when the item is selected in the ListView
// I know.. it's a very sad hack indeed
const static QColor HIGHLIGHTED_FOREGROUND = QColor("#eeeeee");

const static int HEIGHT_EXTRA_PADDING = 20;

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
        ctx.palette.setColor(QPalette::Text, HIGHLIGHTED_FOREGROUND);
    } else {
        ctx.palette.setColor(QPalette::Text, optionV4.palette.color(QPalette::Active, QPalette::WindowText));
    }


    QRect textRect = style->subElementRect(QStyle::SE_ItemViewItemText, &optionV4, optionV4.widget);
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
    doc.setDefaultStyleSheet(this->styleSheet);
    doc.setHtml(optionV4.text);

    // this causes the highlight problem on linux systems
#ifdef _WIN32
    doc.setTextWidth(optionV4.rect.width());
#endif

    auto width = doc.idealWidth();
    auto height = doc.size().rheight() + this->extraPadding;

    return QSize(width, height);
}

AudiobookListDelegate::AudiobookListDelegate(QString styleSheet, int extraPadding) {
    this->styleSheet = styleSheet;
    this->extraPadding = extraPadding;
}

AudiobookListDelegate::AudiobookListDelegate() {
    AudiobookListDelegate("");
}
