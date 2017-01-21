// Original by @Anthony Serdyukov posted on Stackoverflow
// http://stackoverflow.com/a/2039745/596065

#ifndef NODOKANATIVE_AUDIOBOOKLISTDELEGATE_H
#define NODOKANATIVE_AUDIOBOOKLISTDELEGATE_H

#include <QStyledItemDelegate>
#include <QTextDocument>
#include <QPainter>
#include <QApplication>
#include <QAbstractTextDocumentLayout>

class AudiobookListDelegate: public QStyledItemDelegate {
private:
    QString styleSheet;

public:
    AudiobookListDelegate();
    AudiobookListDelegate(QString styleSheet);

protected:
    void paint ( QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index ) const;
    QSize sizeHint ( const QStyleOptionViewItem & option, const QModelIndex & index ) const;
};


#endif //NODOKANATIVE_AUDIOBOOKLISTDELEGATE_H
