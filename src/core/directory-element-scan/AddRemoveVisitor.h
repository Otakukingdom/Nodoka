//
// Created by mistlight on 3/5/2017.
//

#ifndef NODOKA_ADDREMOVEVISITOR_H
#define NODOKA_ADDREMOVEVISITOR_H


#include <src/model/Audiobook.h>
#include "AbstractElementScanner.h"

/**
 * AddRemoveVisitor is called whenever a directory is scanned, or re-scanned
 */
class AddRemoveVisitor : public AbstractElementScanner {
private:
    Audiobook* audiobookModel;

    QDir baseDirectory;

public:
    AddRemoveVisitor(Audiobook* audiobookModel,
                     QDir baseDirectory);

    void accept(const QDir& dir);
    void accept(const QFile& file);
    void accept(QString directory);

    void addRemoveAudiobook(QDir directory, std::vector<QDir> subdirectories, std::vector<QFile> files);
};


#endif //NODOKA_ADDREMOVEVISITOR_H
