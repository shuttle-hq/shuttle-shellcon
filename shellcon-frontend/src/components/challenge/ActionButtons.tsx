
import React from 'react';
import { Button } from "@/components/ui/button";
import { BookOpen, Code, HelpCircle, CheckSquare } from "lucide-react";

interface ActionButtonsProps {
  hasHint: boolean;
  hasSolution: boolean;
  hasLecture: boolean;
  showHint: boolean;
  showSolution: boolean;
  showMoreInfo: boolean;
  onToggleHint: () => void;
  onSolutionRequest: () => void;
  onLectureRequest: () => void;
  onValidateRequest?: () => void;
  isValidating?: boolean;
  challengeId: number; // Changed from string to number
}

const ActionButtons: React.FC<ActionButtonsProps> = ({
  hasHint,
  hasSolution,
  hasLecture,
  showHint,
  showSolution,
  showMoreInfo,
  onToggleHint,
  onSolutionRequest,
  onLectureRequest,
  onValidateRequest,
  isValidating = false,
  challengeId
}) => {
  return (
    <div className="flex flex-wrap gap-3 mb-2">
      {/* Validate Solution Button */}
      {onValidateRequest && (
        <Button
          variant="outline"
          size="sm"
          className="text-green-400 border-green-500 hover:bg-green-500/10 hover:text-green-300"
          onClick={onValidateRequest}
          disabled={isValidating}
        >
          <CheckSquare className="h-4 w-4 mr-1" />
          {isValidating ? "Validating..." : "Validate your solution"}
        </Button>
      )}

      {/* Hint Button */}
      {hasHint && (
        <Button
          variant="outline"
          size="sm"
          className="text-blue-400 border-blue-500 hover:bg-blue-500/10 hover:text-blue-300"
          onClick={onToggleHint}
        >
          <HelpCircle className="h-4 w-4 mr-1" />
          {showHint ? "Hide Hint" : "Show Hint"}
        </Button>
      )}

      {/* Solution Button */}
      {hasSolution && (
        <Button
          variant="outline"
          size="sm"
          className="text-orange-400 border-orange-500 hover:bg-orange-500/10 hover:text-orange-300"
          onClick={onSolutionRequest}
        >
          <Code className="h-4 w-4 mr-1" />
          {showSolution ? "Hide Solution" : "Show Solution"}
        </Button>
      )}

      {/* Teach me how it works Button - only shown if lecture is available */}
      {hasLecture && (
        <Button
          variant="outline"
          size="sm"
          className="text-green-400 border-green-500 hover:bg-green-500/10 hover:text-green-300"
          onClick={onLectureRequest}
        >
          <BookOpen className="h-4 w-4 mr-1" />
          {showMoreInfo ? "Hide Lesson" : "Teach me how it works"}
        </Button>
      )}
    </div>
  );
};

export default ActionButtons;
