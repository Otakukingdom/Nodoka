//
// Created by mistlight on 3/5/2017.
//

#ifndef NODOKA_ADDREMOVEVISITOR_H
#define NODOKA_ADDREMOVEVISITOR_H


#include <src/model/Audiobook.h>
#include "AbstractElementScanner.h"
#include <QDir>
#include <QFile>

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

    void accept(const std::shared_ptr<QDir>& dir);
    void accept(const std::shared_ptr<QFile>& file);
    void accept(const QString directory);

    void addRemoveAudiobook(const std::shared_ptr<QDir>& directory,
                            const std::vector<std::shared_ptr<QDir>>& subdirectories,
                            const std::vector<std::shared_ptr<QFile>>& files);
};


#endif //NODOKA_ADDREMOVEVISITOR_H
